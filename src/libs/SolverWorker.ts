onmessage = async (e) => {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "web") {
        var { dfs_solve, nq_solve, rika_solve } = await import("../../pkg-wasm/app_wasm")
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
        case "rika_solve_tnzever":
        case "reflect_solve":
            throw "unsupported"
    }
    close()
};
