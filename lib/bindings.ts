export const library = Deno.dlopen("./target/debug/plotsaur.dll", {
    ops_create_window: {
        parameters: ["pointer", "usize", "f64", "f64"], 
        result: "void"
    },
    ops_build_plot: {
        parameters: ["pointer", "usize"], 
        result: "void"
    },
    ops_write_data: {
        parameters: ["pointer", "usize", "pointer", "usize"], 
        result: "void"
    },
    ops_run_return: {
        parameters: [], 
        result: "usize"
    }
})