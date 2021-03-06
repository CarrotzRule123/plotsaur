import { COLOR } from "../mod.ts";
import { Cartesian2D, ShapeColor, TextStyle } from "./types.ts";
import { createObject } from "./utils.ts";

export interface PlotChart {
    margin?: number
    caption?: {
        caption: string,
        style: TextStyle
    }
    xLabelAreaSize?: number,
    yLabelAreaSize?: number,
    cartesian2D?: Cartesian2D,
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
    margin?= 20;
    xLabelAreaSize?= 40;
    yLabelAreaSize?= 40;
    mesh?: PlotChartMesh = {
        xLabels: 10,
        yLabels: 10,
        axisDescStyle: {
            family: "sans-serif",
            size: 15
        }
    };
    seriesLabel?: SeriesLabel = {
        backgroundStyle: COLOR.WHITE,
        borderStyle: COLOR.BLACK
    }

    constructor(options: Partial<PlotChart>) {
        Object.assign(this, options)
    }

    public build() {
        const options = []
        for (const key in this) {
            // deno-lint-ignore no-explicit-any
            let value: any = this[key]
            if (key == "mesh" || key == "seriesLabel") {
                value = []
                for (const key2 in this[key]) {
                    if (this[key][key2]) {
                        value.push(createObject(key2, this[key][key2]))
                    }
                }
            }
            if (this[key]) options.push(createObject(key, value))
        }
        return JSON.stringify({ options })
    }
}