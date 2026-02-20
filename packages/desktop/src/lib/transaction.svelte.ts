// Copyright (C) 2025 Wakunguma Kalimukwa
// SPDX-License-Identifier: GPL-3.0-or-later
import { parseDate } from "@internationalized/date";
import { invoke } from "@tauri-apps/api/core";
import { parse } from "date-fns";

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
        const transaction = await invoke<Transaction>("edit_transaction",{data:opts});
        const index = this.#transactions.findIndex(t => t.id === transaction.id);
        // let transactions = this.#transactions.filter(t => t.id !== transaction.id);
        // transactions.push(transaction);
        this.#transactions[index] = transaction;
        // transactions
        //     .sort((a,b) => new Date(a.transactionDate).getTime()-new Date(b.transactionDate).getTime())
        //     .reverse();
        // this.#transactions = transactions;
    }
    
    async load(){
        let transactions = await invoke<Transaction[]>("fetch_transactions");
        transactions
            .sort((a,b) => new Date(a.transactionDate).getTime()-new Date(b.transactionDate).getTime())
            .reverse();
        this.#transactions = transactions;
    }
}

export const transactionStore = new TransactionStore();

