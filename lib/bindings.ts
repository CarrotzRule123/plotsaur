export const library = Deno.dlopen("./target/debug/plotsaur.dll", {
    ops_create_window: {
        parameters: ["pointer", "usize"], 
        result: "void"
    },
    ops_run_return: {
        parameters: [], 
        result: "usize"
    }
})