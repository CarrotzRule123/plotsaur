import { library } from "../bindings.ts";
import { Range, ShapeColor, TextStyle } from "../types.ts";
import { createObject } from "../utils.ts";

export interface PlotChart {
    margin?: number
    caption?: {
        caption: string,
        style: TextStyle
    }
    xLabelAreaSize?: number,
    yLabelAreaSize?: number,
    cartesian2D: {
        x_axis: Range,
        y_axis: Range
    },
    mesh?: PlotChartMesh,
    seriesLabel?: SeriesLabel
}

type PlotChartMesh = {
    xLabels?: number,
    yLabels?: number,
    xDesc?: string,
    yDesc?: string,
    axisDescStyle?: TextStyle,
}

type SeriesLabel = {
    backgroundStyle?: ShapeColor,
    borderStyle?: ShapeColor,
}

export class PlotChart {
    constructor(options: Partial<PlotChart>) {
        Object.assign(this, options)
    }

    public build() {
        const chart = []
        for (const key in this) {
            let value: any = this[key]
            if (key == "mesh" || key == "seriesLabel") {
                value = []
                for (const key2 in this[key]) {
                    if (this[key][key2]) {
                        value.push(createObject(key2, this[key][key2]))
                    }
                }
            }
            if (this[key]) chart.push(createObject(key, value))
        }
        return JSON.stringify({ chart })
    }

    public plot(color: ShapeColor, label: string, values: number[]) {
        const json = JSON.stringify({ color, label })
        const buf = new TextEncoder().encode(json)
        const data = new Float64Array(values)
        library.symbols.ops_write_data(buf, buf.length, data, data.length)
    }
}