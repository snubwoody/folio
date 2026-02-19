// Copyright (C) 2025 Wakunguma Kalimukwa
// SPDX-License-Identifier: GPL-3.0-or-later
import { invoke } from "@tauri-apps/api/core";

export interface Transaction{
    id: string,
    amount: string,
    from_account_id?: string,
    to_account_id?: string,
    transaction_date: string,
    category_id?: string,
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

