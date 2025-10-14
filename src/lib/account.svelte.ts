import { invoke } from "@tauri-apps/api/core";
import type { Account } from "./lib";

export class AccountStore{
    accounts: Account[] = $state([]);

    async addAccount(name: string, startingBalance: string){
        try{
            await invoke("create_account",{ name,startingBalance });
        }
        catch(e){
            console.error(e);
        }
        await this.load();
    }

    async load(){
        // FIXME: check for error
        const accounts = await invoke("fetch_accounts") as Account[];
        this.accounts = accounts;
    }
}

export const accountStore = new AccountStore();
