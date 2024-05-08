onmessage = async (e) => {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "web") {
        var { rand_simulation } = await import("../../pkg-wasm/app_wasm")
    } else return
    const { name, args: argsJson } = e.data
    const args = JSON.parse(argsJson)
    switch (name) {
        case "rand_simulation":
            postMessage(rand_simulation(args.status, args.actions, args.n))
            break
    }
    close()
};
