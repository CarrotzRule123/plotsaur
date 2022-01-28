export function createObject<T>(key: string, value: T) {
    const object: {[key: string]: T} = {}
    object[key] = value
    return object
}