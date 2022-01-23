use iced_wgpu::{wgpu, Backend, Renderer, Settings, Viewport};
use iced_winit::futures::task::SpawnExt;
use iced_winit::program::State;
use iced_winit::winit::{
    dpi::PhysicalPosition,
    event::{Event, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};
use iced_winit::{conversion, futures, Clipboard, Debug, Size};

use super::plot::PlotProgram;

pub struct PlotWindow {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain: wgpu::SwapChain,
    pub format: wgpu::TextureFormat,
    pub staging_belt: wgpu::util::StagingBelt,

    pub viewport: Viewport,
    pub clipboard: Clipboard,
    pub renderer: Renderer,
    pub debug: Debug,
    pub state: State<PlotProgram>,

    pub cursor_position: PhysicalPosition<f64>,
    pub modifiers: ModifiersState,
    pub resized: bool,

    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub local_pool: futures::executor::LocalPool,
    // program: PlotProgram
}

impl PlotWindow {
    pub fn new(plot_program: PlotProgram) -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        let size = window.inner_size();
        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        let viewport =
            Viewport::with_physical_size(Size::new(size.width, size.height), window.scale_factor());

        // handle wgpu
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let surface = unsafe { instance.create_surface(&window) };
        let (format, (mut device, queue)) = futures::executor::block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                })
                .await
                .expect("Request adapter");
            (
                adapter.get_swap_chain_preferred_format(&surface),
                adapter
                    .request_device(
                        &wgpu::DeviceDescriptor {
                            features: wgpu::Features::empty(),
                            limits: wgpu::Limits::default(),
                            label: None,
                        },
                        None, // Trace path
                    )
                    .await
                    .unwrap(),
            )
        });
        let staging_belt = wgpu::util::StagingBelt::new(5 * 1024);
        let local_pool = futures::executor::LocalPool::new();
        let config = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &config);

        // handle iced
        let mut debug = Debug::new();
        let mut renderer = Renderer::new(Backend::new(&mut device, Settings::default()));
        let position = conversion::cursor_position(cursor_position, viewport.scale_factor());
        let clipboard = Clipboard::connect(&window);
        let resized = false;
        let modifiers = ModifiersState::default();
        let state = State::new(
            plot_program,
            viewport.logical_size(),
            position,
            &mut renderer,
            &mut debug,
        );
        

        Self {
            surface,
            device,
            queue,
            swap_chain,
            format,
            staging_belt,
            viewport,
            clipboard,
            renderer,
            debug,
            state,
            cursor_position,
            modifiers,
            resized,
            event_loop,
            window,
            local_pool,
        }
    }

    pub fn run_return(&mut self) {
        self.event_loop.run_return(|event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            self.cursor_position = position;
                        }
                        WindowEvent::ModifiersChanged(new_modifiers) => {
                            self.modifiers = new_modifiers;
                        }
                        WindowEvent::Resized(new_size) => {
                            self.viewport = Viewport::with_physical_size(
                                Size::new(new_size.width, new_size.height),
                                self.window.scale_factor(),
                            );
                            self.resized = true;
                        }
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }
                    // Map window event to iced event
                    if let Some(event) =
                        conversion::window_event(&event, self.window.scale_factor(), self.modifiers)
                    {
                        self.state.queue_event(event);
                    }
                }
                Event::MainEventsCleared => {
                    // If there are events pending
                    if !self.state.is_queue_empty() {
                        // We update iced
                        let _ = self.state.update(
                            self.viewport.logical_size(),
                            conversion::cursor_position(
                                self.cursor_position,
                                self.viewport.scale_factor(),
                            ),
                            &mut self.renderer,
                            &mut self.clipboard,
                            &mut self.debug,
                        );
                        // and request a redraw
                        self.window.request_redraw();
                    }
                }
                Event::RedrawRequested(_) => {
                    if self.resized {
                        let size = self.window.inner_size();
                        self.swap_chain = self.device.create_swap_chain(
                            &self.surface,
                            &wgpu::SwapChainDescriptor {
                                usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                                format: self.format,
                                width: size.width,
                                height: size.height,
                                present_mode: wgpu::PresentMode::Mailbox,
                            },
                        );
                        self.resized = false;
                    }
                    match self.swap_chain.get_current_frame() {
                        Ok(frame) => {
                            let mut encoder = self.device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor { label: None },
                            );
                            let program = self.state.program();
                            {
                                // We clear the frame
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[
                                        wgpu::RenderPassColorAttachmentDescriptor {
                                            attachment: &frame.output.view,
                                            resolve_target: None,
                                            ops: wgpu::Operations {
                                                load: wgpu::LoadOp::Clear({
                                                    wgpu::Color {
                                                        r: 255.0,
                                                        g: 255.0,
                                                        b: 255.0,
                                                        a: 255.0,
                                                    }
                                                }),
                                                store: true,
                                            },
                                        },
                                    ],
                                    depth_stencil_attachment: None,
                                });
                            }
                            // And then iced on top
                            let mouse_interaction = self.renderer.backend_mut().draw(
                                &mut self.device,
                                &mut self.staging_belt,
                                &mut encoder,
                                &frame.output.view,
                                &self.viewport,
                                self.state.primitive(),
                                &self.debug.overlay(),
                            );
                            // Then we submit the work
                            self.staging_belt.finish();
                            self.queue.submit(Some(encoder.finish()));
                            // Update the mouse cursor
                            self.window
                                .set_cursor_icon(iced_winit::conversion::mouse_interaction(
                                    mouse_interaction,
                                ));
                            // And recall staging buffers
                            self.local_pool
                                .spawner()
                                .spawn(self.staging_belt.recall())
                                .expect("Recall staging buffers");
                            self.local_pool.run_until_stalled();
                        }
                        Err(error) => match error {
                            wgpu::SwapChainError::OutOfMemory => {
                                panic!("Swapchain error: {}. Rendering cannot continue.", error)
                            }
                            _ => {
                                // Try rendering again next frame.
                                self.window.request_redraw();
                            }
                        },
                    }
                }
                _ => {}
            }
        })
    }
}
