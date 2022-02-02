import { Point } from "./types.ts";

export function createObject<T>(key: string, value: T) {
    const object: { [key: string]: T } = {}
    object[key] = value
    return object
}

export function scaleShapes(points: Point[]) {
    // ...I have no idea why this works 
    // plotters just happens to shrink shapes by 0.2 times
    points.map(point => scalePoint(point))
}

export function scalePoint(point: Point) {
    point.x = Math.round(point.x * 1.25);
    point.y = Math.round(point.y * 1.25)
}