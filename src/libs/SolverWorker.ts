onmessage = async (e) => {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "web") {
        var { dfs_solve, nq_solve } = await import("../../pkg-wasm/app_wasm")
    } else return
    const { name, args } = e.data
    switch (name) {
        case "dfs_solve":
        case "nq_solve":
            const { status, depth, specialist } = JSON.parse(args)
            let solve = name == "dfs_solve" ? dfs_solve : nq_solve
            const result = solve(status, depth, specialist)
            postMessage(result)
            break;
        case "rika_solve":
        case "rika_solve_tnzever":
        case "reflect_solve":
            throw "unsupported"
    }
    close()
};
