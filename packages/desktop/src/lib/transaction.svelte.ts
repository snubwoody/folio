// Copyright (C) 2025 Wakunguma Kalimukwa
// SPDX-License-Identifier: GPL-3.0-or-later
import { invoke } from "@tauri-apps/api/core";

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

    async load(){
        this.#transactions = await invoke("fetch_transactions");
    }
}

export const transactionStore = new TransactionStore();

