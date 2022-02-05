import { Plug } from "../deps.ts";

const options: Plug.Options = {
    name: "plotsaur",
    url: "./release/",
    policy: "NONE",
    // url: "./target/release/",
}

export const library = await Plug.prepare(options, {
    ops_create_window: {
        parameters: ["pointer", "usize", "f64", "f64"], 
        result: "void"
    },
    ops_build_plot: {
        parameters: ["pointer", "usize"], 
        result: "void"
    },
    ops_draw_element: {
        parameters: ["pointer", "usize"], 
        result: "void"
    },
    ops_draw_series: {
        parameters: ["pointer", "usize", "pointer", "usize"], 
        result: "void"
    },
    ops_run_return: {
        parameters: [], 
        result: "usize"
    }
})