import { library } from "./bindings.ts"
import { PlotChart } from "./chart.ts";
import { SeriesOptions } from "./types.ts";

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

    public addPlot(options: Partial<PlotChart>) {
        const json = new PlotChart(options).build()
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

    public plotSeries(options: SeriesOptions, values: number[]) {
        const json = JSON.stringify({ series: { ...options } })
        const buf = this.encoder.encode(json)
        const data = new Float64Array(values)
        library.symbols.ops_write_data(buf, buf.length, data, data.length)
    }

    // public plotHistogram(options: SeriesOptions, values: number[]) {
    //     const json = JSON.stringify({ series: { ...options } })
    //     const buf = this.encoder.encode(json)
    //     const data = new Float64Array(values)
    //     library.symbols.ops_write_data(buf, buf.length, data, data.length)
    // }
}