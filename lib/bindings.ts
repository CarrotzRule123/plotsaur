Deno.dlopen("./target/debug/plotsaur.dll", {
    ops_create_window: {
        parameters: [], 
        result: "u8"
    },
    ops_run_return: {
        parameters: [], 
        result: "u8"
    }
})