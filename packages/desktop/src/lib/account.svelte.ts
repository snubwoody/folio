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
import type { Transaction } from "./transaction.svelte";

interface EditAccount{
    name?: string
    startingBalance?: string
}

export class AccountStore{
    #accounts: Account[] = $state([]);
    #accountMap: SvelteMap<string,Account> = $derived(new SvelteMap(this.accounts.map(a => [a.id,a])));

    get accounts(): Account[]{
        return this.#accounts;
    }

    get accountMap(){
        return this.#accountMap;
    }

    /**
     * Returns the account balance
     * @param id The id of the account
     */
    async accountBalance(id:string):Promise<string>{
        const balance = await invoke<string>("account_balance", { id });
        return balance;
    }

    /**
     * Create a new account
     * @param name The name of the account
     * @param startingBalance The starting balance
     */
    async createAccount({ name,startingBalance }:{name: string, startingBalance?: string}): Promise<Account> {
        const balance = startingBalance ?? "0";
        const account = await invoke<Account>("create_account", { name, startingBalance: balance });
        this.#accounts.push(account);
        return account;
    }

    async editAccount(id: string, opts: EditAccount) {
        const account = await invoke<Account>("edit_account", { id, opts });
        const index = this.#accounts.findIndex(
            (a) => a.id === account.id
        );
        this.#accounts[index] = account;
    }

    async deleteAccount(id: string) {
        await invoke("delete_account",{ id });
        this.#accounts = this.#accounts.filter(a => a.id !== id);
    }

    async createTestAccount({ name }:{name:string}):Promise<Account>{
        const id = Math.random().toString(36).slice(2);
        const account: Account = {
            id,
            name,
            startingBalance: "0.0",
            balance: "0.0"
        };
        this.#accounts.push(account);
        return account;
    }

    clear(){
        this.#accounts = [];
    }

    /// Loads the accounts from the backend
    async load(){
        this.#accounts = await invoke("fetch_accounts");
    }
}

export const accountStore = new AccountStore();

/**
 * Calculates the account balance
 *
 * @param accountId The id of the account
 * @param transactions The list of transactions
 * @returns The account balance
 */
export function accountBalance(accountId:string,transactions: Transaction[]): number{
    let total = 0;
    transactions
        .filter(t => t.toAccountId === accountId)
        .forEach(t => total += parseFloat(t.amount));
    transactions
        .filter(t => t.fromAccountId === accountId)
        .forEach(t => total -= parseFloat(t.amount));
    return total;
}