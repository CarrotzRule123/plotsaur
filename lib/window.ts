import { library } from "./bindings.ts"
import { Plot } from "./types.ts";

export class PlotWindow {
    private encoder: TextEncoder
    public title: string
    public height: number
    public width: number

    constructor(title: string, height: number, width: number) {
        this.encoder = new TextEncoder()
        this.title = title
        this.height = height
        this.width = width
    }

    public addPlot(plot: Plot) {
        const json = plot.build()
        const buffer = new Uint8Array(this.encoder.encode(json))
        library.symbols.ops_build_plot(buffer, buffer.length)
    }

    public show() {
        const { width, height } = this
        const buffer = new Uint8Array(this.encoder.encode(this.title))
        library.symbols.ops_create_window(buffer, buffer.length, width, height)
        while (true) {
            const control = library.symbols.ops_run_return()
            if (control == 1) {
                break
            }
        }
    }
}