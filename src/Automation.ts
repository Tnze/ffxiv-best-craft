import { invoke } from "@tauri-apps/api/tauri";

const startHttpServer = (addr: string): Promise<void> => {
    return invoke("start_http_server", { addr });
};

export { startHttpServer }