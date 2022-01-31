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