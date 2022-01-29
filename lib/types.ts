import { PlotChart } from "./plots/chart.ts";

export type Plot = PlotChart

export type TextStyle = {
    family: Font,
    size: number
}

export type Range = {
    start: number,
    end: number
}

export type ShapeColor = {
    r: number,
    g: number,
    b: number,
    a: number
}

export type Font = "serif" | "sans-serif" | "monospace"