onmessage = async (e) => {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "web") {
        var { dfs_solve, nq_solve, rika_solve, reflect_solve, raphael_solve } = await import("../../pkg-wasm/app_wasm")
    } else return
    const { name, args: argsJson } = e.data
    const args = JSON.parse(argsJson)
    switch (name) {
        case "dfs_solve":
        case "nq_solve":
            let solve = name == "dfs_solve" ? dfs_solve : nq_solve
            postMessage(solve(args.status, args.depth, args.specialist))
            break;
        case "rika_solve":
            postMessage(rika_solve(args.status))
            break;
        case "reflect_solve":
            postMessage(reflect_solve(args.status, args.useObserve))
            break;
        case "rika_solve_tnzever":
            throw "unsupported";
        case "raphael_solve":
            postMessage(raphael_solve(args.status, args.useManipulation, args.useHeartAndSoul, args.useQuickInnovation, args.backloadProgress))
    }
    close()
};
