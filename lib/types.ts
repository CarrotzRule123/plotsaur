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

export const COLOR = Object.freeze({
    RED: { r: 255, g: 0, b: 0, a: 1 },
    YELLOW: { r: 255, g: 255, b: 0, a: 1 },
    GREEN: { r: 0, g: 255, b: 0, a: 1 },
    BLUE: { r: 0, g: 0, b: 255, a: 1 },
    BLACK: { r: 0, g: 0, b: 0, a: 1 },
    WHITE: { r: 255, g: 255, b: 255, a: 1 },
})

export type Font = "serif" | "sans-serif" | "monospace"

export type Point = {
    x: number,
    y: number
}

export type SeriesOptions = {
    color: ShapeColor, 
    label: string
}

export type Rect = {
    points: Point[],
    style: ShapeColor,
    filled: boolean
}

export type Circle = {
    points: Point,
    size: number,
    style: ShapeColor,
    filled: boolean
}

export type Polygon = {
    points: Point[],
    style: ShapeColor,
    filled: boolean
}

export type Text = {
    points: Point,
    style: TextStyle,
    color: ShapeColor,
    text: string
}

export type Cartesian2D = {
    type: ChartType,
    x_axis: Range | string[],
    y_axis: Range | string[]
}

export type Histogram = {
    color: ShapeColor, 
    filled: boolean
}

export type HistogramOptions = {
    color: ShapeColor, 
    filled: boolean
}

export type Path = {
    points: Point[],
    style: ShapeColor
}

export type ChartType = "ranged" | "segmentedX" | "segmentedY" | "valuesX" | "valuesY"