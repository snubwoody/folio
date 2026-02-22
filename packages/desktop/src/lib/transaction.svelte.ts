// Copyright (C) 2025 Wakunguma Kalimukwa
// SPDX-License-Identifier: GPL-3.0-or-later
import { invoke } from "@tauri-apps/api/core";
import { SvelteDate } from "svelte/reactivity";

export interface EditTransaction{
    id: string,
    fromAccountId?: string,
    note?: string,
    categoryId?: string,
    transactionDate?: string,
    amount?: string,
    toAccountId?:string
}

export interface Transaction{
    id: string,
    amount: string,
    fromAccountId?: string,
    toAccountId?: string,
    transactionDate: string,
    categoryId?: string,
    note?: string,
}

export class TransactionStore{
    #transactions: Transaction[] = $state([]);

    get transactions(){
        return this.#transactions;
    }

    async editTransaction(opts: EditTransaction){
        const transaction = await invoke<Transaction>("edit_transaction",{ data:opts });
        const index = this.#transactions.findIndex(t => t.id === transaction.id);
        this.#transactions[index] = transaction;
        // TODO: resort because of date
    }

    async load(){
        let transactions = await invoke<Transaction[]>("fetch_transactions");
        transactions
            .sort((a,b) => new SvelteDate(a.transactionDate).getTime()-new SvelteDate(b.transactionDate).getTime())
            .reverse();
        this.#transactions = transactions;
    }
}

export const transactionStore = new TransactionStore();

