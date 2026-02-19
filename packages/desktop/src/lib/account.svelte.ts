// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
import { invoke } from "@tauri-apps/api/core";
import type { Account } from "./lib";
import { SvelteMap } from "svelte/reactivity";

export class AccountStore{
    #accounts: Account[] = $state([]);
    #accountMap: SvelteMap<string,Account> = $derived(new SvelteMap(this.accounts.map(a => [a.id,a])));

    get accounts(): Account[]{
        return this.#accounts;
    }
    
    get accountMap(){
        return this.#accountMap;
    }

    /// Loads the accounts from the backend
    async load(){
        this.#accounts = await invoke("fetch_accounts");
    }
}

export const accountStore = new AccountStore();