use iced_wgpu::{wgpu, Backend, Renderer, Settings, Viewport};
use iced_winit::futures::task::SpawnExt;
use iced_winit::program::State;
use iced_winit::{conversion, futures, Clipboard, Debug, Size};
use iced_winit::winit::{
    dpi::PhysicalPosition,
    event::{Event, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use super::chart::ChartProgram;

pub fn window(chart: ChartProgram) {
    // winit stuff
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let physical_size = window.inner_size();
    let mut cursor_position = PhysicalPosition::new(-1.0, -1.0);

    let mut viewport = Viewport::with_physical_size(
        Size::new(physical_size.width, physical_size.height),
        window.scale_factor(),
    );

    // wgpu stuff
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
    let mut staging_belt = wgpu::util::StagingBelt::new(5 * 1024);
    let mut local_pool = futures::executor::LocalPool::new();

    let mut swap_chain = {
        let size = window.inner_size();

        device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                format: format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Mailbox,
            },
        )
    };

    // iced stuff
    let mut debug = Debug::new();
    let mut renderer = Renderer::new(Backend::new(&mut device, Settings::default()));
    let mut modifiers = ModifiersState::default();
    let cursor_point = conversion::cursor_position(cursor_position, viewport.scale_factor());
    let mut clipboard = Clipboard::connect(&window);
    let mut resized = false;
    let mut state = State::new(
        chart,
        viewport.logical_size(),
        cursor_point,
        &mut renderer,
        &mut debug,
    );

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_position = position;
                    }
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        modifiers = new_modifiers;
                    }
                    WindowEvent::Resized(new_size) => {
                        viewport = Viewport::with_physical_size(
                            Size::new(new_size.width, new_size.height),
                            window.scale_factor(),
                        );

                        resized = true;
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }

                // Map window event to iced event
                if let Some(event) =
                    conversion::window_event(&event, window.scale_factor(), modifiers)
                {
                    state.queue_event(event);
                }
            }
            Event::MainEventsCleared => {
                // If there are events pending
                if !state.is_queue_empty() {
                    // We update iced
                    let _ = state.update(
                        viewport.logical_size(),
                        conversion::cursor_position(cursor_position, viewport.scale_factor()),
                        &mut renderer,
                        &mut clipboard,
                        &mut debug,
                    );

                    // and request a redraw
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                if resized {
                    let size = window.inner_size();

                    swap_chain = device.create_swap_chain(
                        &surface,
                        &wgpu::SwapChainDescriptor {
                            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                            format: format,
                            width: size.width,
                            height: size.height,
                            present_mode: wgpu::PresentMode::Mailbox,
                        },
                    );

                    resized = false;
                }

                match swap_chain.get_current_frame() {
                    Ok(frame) => {
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });

                        let program = state.program();

                        {
                            // We clear the frame
                            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: None,
                                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
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
                                }],
                                depth_stencil_attachment: None,
                            });
                        }

                        // And then iced on top
                        let mouse_interaction = renderer.backend_mut().draw(
                            &mut device,
                            &mut staging_belt,
                            &mut encoder,
                            &frame.output.view,
                            &viewport,
                            state.primitive(),
                            &debug.overlay(),
                        );
                        // Then we submit the work
                        staging_belt.finish();
                        queue.submit(Some(encoder.finish()));

                        // Update the mouse cursor
                        window.set_cursor_icon(iced_winit::conversion::mouse_interaction(
                            mouse_interaction,
                        ));

                        // And recall staging buffers
                        local_pool
                            .spawner()
                            .spawn(staging_belt.recall())
                            .expect("Recall staging buffers");

                        local_pool.run_until_stalled();
                    }
                    Err(error) => match error {
                        wgpu::SwapChainError::OutOfMemory => {
                            panic!("Swapchain error: {}. Rendering cannot continue.", error)
                        }
                        _ => {
                            // Try rendering again next frame.
                            window.request_redraw();
                        }
                    },
                }
            }
            _ => {}
        }
    })
}
