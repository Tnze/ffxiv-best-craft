onmessage = async (e) => {
    const { name, args } = e.data
    switch (name) {
        case "dfs_solve":
        case "nq_solve":
            const { status, depth, specialist } = JSON.parse(args)
            let { dfs_solve } = await import("../../pkg-wasm/app_wasm")
            let solve = name == "dfs_solver" ? dfs_solve : dfs_solve
            const result = solve(status, depth, specialist)
            postMessage(result)
            break;
        case "rika_solve":
        case "rika_solve_tnzever":
        case "reflect_solve":
            // throw "unsupported"
    }
    // close()
};
