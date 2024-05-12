onmessage = async (e) => {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "web") {
        var { rand_simulation, calc_attributes_scope } = await import("../../pkg-wasm/app_wasm")
    } else return
    const { name, args: argsJson } = e.data
    const args = JSON.parse(argsJson)
    try {
        switch (name) {
            case "rand_simulation":
                postMessage(rand_simulation(args.status, args.actions, args.n, args.ignoreErrors))
                break
            case "calc_attributes_scope":
                postMessage(calc_attributes_scope(args.status, args.actions))
        }
    } catch (e) {
        throw e
    }
    close()
};
