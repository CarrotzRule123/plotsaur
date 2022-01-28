import { createObject } from "../utils.ts";

export class PlotChart {
    public margin?: number
    public caption?: {
        caption: string,
        style: {
            family: string,
            size: number
        }
    }

    constructor(options: Partial<PlotChart>) {
        Object.assign(this, options)
    }

    public build() {
        const chart = []
        for (const key in this) {
            if (this[key]) {
                chart.push(createObject(key, this[key]))
            }
        }
        return JSON.stringify({ chart })
    }
}