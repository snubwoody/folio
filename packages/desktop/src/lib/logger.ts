import { invoke } from "@tauri-apps/api/core";

/**
 * Global logger
 */
export const logger = {
    info(message: string){
        // TODO: forward console
        void invoke("log_info",{ message });
    },
    error(message: string){
        void invoke("log_error",{ message });
    },
    warn(message: string){
        void invoke("log_warn",{ message });
    },
    debug(message: string){
        void invoke("log_debug",{ message });
    }
};
