import { library } from "./bindings.ts"

export class PlotWindow {
    private title: Uint8Array

    constructor(title: string) {
        const encoder = new TextEncoder()
        this.title = encoder.encode(title);
    }

    public addChart() {
    }

    public show() {
        library.symbols.ops_create_window(this.title, this.title.length)
        while (true) {
            const control = library.symbols.ops_run_return()
            if (control == 1) {
                break
            }
        }
    }
}