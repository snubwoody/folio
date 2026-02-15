import { invoke } from "@tauri-apps/api/core";

export const logger = {
    info(message: string){
        // TODO: forward console
        invoke("log_info",{ message });
    },
    error(message: string){
        invoke("log_error",{ message });
    },
    warn(message: string){
        invoke("log_warn",{ message });
    },
    debug(message: string){
        invoke("log_debug",{ message });
    }
};
