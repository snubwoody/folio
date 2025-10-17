import { invoke } from "@tauri-apps/api/core";
import type { AppStore } from "./state.svelte";

export class AccountStore{
    #rootStore: AppStore;

    constructor(store: AppStore){
        this.#rootStore = store;
    }

    async addAccount(name: string, startingBalance: string){
        console.log(startingBalance);
        try{
            await invoke("create_account",{ name,startingBalance });
        }
        catch(e){
            console.error(e);
        }
        await this.#rootStore.load();
    }

}

