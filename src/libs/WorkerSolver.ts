onmessage = async (e) => {
    console.debug("worker running")
    const { name, args } = e.data
    console.debug(name, args)
    switch (name) {
        case "dfs_solve":
            try {
                console.log("start running dfs solver")
                const { status, depth, specialist } = JSON.parse(args)
                console.debug(status, depth, specialist)
                let a = await import("../../pkg-wasm/app_wasm")
                console.log(a.dfs_solve)
                const result = a.dfs_solve(status, depth, specialist)
                console.debug(result)
                postMessage(result)
            } catch (err) {
                console.error(err)
            }
            break;
    }
};
