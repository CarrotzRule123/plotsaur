import { library } from "./bindings.ts"
import { PlotChart } from "./chart.ts";
import { Cartesian2D, Circle, HistogramOptions, Path, Polygon, Rect, SeriesOptions, Text } from "./types.ts";
import { scalePoint, scaleShapes } from "./utils.ts";

export class PlotWindow {
    private encoder: TextEncoder
    public title: string
    public height: number
    public width: number
    public chart?: PlotChart

    constructor(title: string, height: number, width: number) {
        this.encoder = new TextEncoder()
        this.title = title
        this.height = height
        this.width = width
    }

    public addPlot(options: Partial<PlotChart>) {
        this.chart = new PlotChart(options)
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

    public cartesian2D(range: Cartesian2D) {
        if (this.chart) {
            this.chart.cartesian2D = range
            const json = this.chart.build()
            const buffer = new Uint8Array(this.encoder.encode(json))
            library.symbols.ops_build_plot(buffer, buffer.length)
        }
    }

    public plotLineSeries(options: SeriesOptions, data: number[]) {
        const json = JSON.stringify({ line: { ...options } })
        const buf = this.encoder.encode(json)
        const vec = new Float64Array(data)
        library.symbols.ops_draw_series(buf, buf.length, vec, vec.length)
    }

    public drawRect(options: Rect) {
        scaleShapes(options.points)
        const json = JSON.stringify({ rect: { ...options } })
        const buf = this.encoder.encode(json)
        library.symbols.ops_draw_element(buf, buf.length)
    }

    public drawCircle(options: Circle) {
        const json = JSON.stringify({ circle: { ...options } })
        const buf = this.encoder.encode(json)
        library.symbols.ops_draw_element(buf, buf.length)
    }

    public drawPolygon(options: Polygon) {
        scaleShapes(options.points)
        const json = JSON.stringify({ polygon: { ...options } })
        const buf = this.encoder.encode(json)
        library.symbols.ops_draw_element(buf, buf.length)
    }

    public drawText(options: Text) {
        scalePoint(options.points)
        const json = JSON.stringify({ text: { ...options } })
        const buf = this.encoder.encode(json)
        library.symbols.ops_draw_element(buf, buf.length)
    }

    public drawPath(options: Path) {
        scaleShapes(options.points)
        const json = JSON.stringify({ path: { ...options } })
        const buf = this.encoder.encode(json)
        library.symbols.ops_draw_element(buf, buf.length)
    }

    public plotHistogram(options: HistogramOptions, data: number[]) {
        const json = JSON.stringify({ histogram: { ...options } })
        const buf = this.encoder.encode(json)
        const vec = new Float64Array(data)
        library.symbols.ops_draw_series(buf, buf.length, vec, vec.length)
    }
}