onmessage = async e => {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == 'web') {
        var { dfs_solve, nq_solve, rika_solve, reflect_solve, raphael_solve } =
            await import('../../pkg-wasm/app_wasm');
    } else return;
    const { name, args: argsJson } = e.data;
    const args = JSON.parse(argsJson);
    try {
        var result;
        switch (name) {
            case 'dfs_solve':
            case 'nq_solve':
                let solve = name == 'dfs_solve' ? dfs_solve : nq_solve;
                result = solve(args.status, args.depth, args.specialist);
                break;
            case 'rika_solve':
                result = rika_solve(args.status);
                break;
            case 'reflect_solve':
                result = reflect_solve(args.status, args.useObserve);
                break;
            case 'rika_solve_tnzever':
                throw 'unsupported';
            case 'raphael_solve':
                result = raphael_solve(
                    args.status,
                    args.targetQuality,
                    args.useManipulation,
                    args.useHeartAndSoul,
                    args.useQuickInnovation,
                    args.useTrainedEye,
                    args.backloadProgress,
                    args.adversarial,
                );
        }
        postMessage(result);
    } catch (e: any) {
        postMessage({ error: String(e) });
    } finally {
        close();
    }
};
