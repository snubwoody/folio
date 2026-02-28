// Copyright (C) 2025 Wakunguma Kalimukwa
// SPDX-License-Identifier: GPL-3.0-or-later
import { invoke } from "@tauri-apps/api/core";
import { SvelteDate } from "svelte/reactivity";
import { logger } from "./logger";

export interface EditTransaction {
    id: string;
    fromAccountId?: string;
    note?: string;
    categoryId?: string;
    transactionDate?: string;
    amount?: string;
    toAccountId?: string;
}

export interface Transaction {
    id: string;
    amount: string;
    fromAccountId?: string;
    toAccountId?: string;
    transactionDate: string;
    categoryId?: string;
    note?: string;
}

export type TransactionType = "Expense" | "Income" | "Transfer";

/**
 * Returns the type of transaction
 * @param transaction
 */
export const transactionType = (transaction: Transaction): TransactionType => {
    if (
        transaction.toAccountId === null ||
        transaction.toAccountId === undefined
    ) {
        return "Expense";
    }

    if (
        transaction.fromAccountId === null ||
        transaction.fromAccountId === undefined
    ) {
        return "Income";
    }

    return "Transfer";
};

export class TransactionStore {
    #transactions: Transaction[] = $state([]);

    get transactions() {
        return this.#transactions;
    }

    async deleteTransactions(ids: string[]) {
        // TODO: resort
        await invoke("delete_transactions", { ids });
        this.#transactions = this.#transactions.filter(t => !ids.includes(t.id));
    }

    /**
     * Creates a new expense
     * @param amount The amount
     * @param date The date the expense occurred
     * @param note An optional note for additional context
     */
    async createExpense({
        amount = "0.0",
        date,
        note,
        account
    }: {
        amount: string;
        date: string;
        note?: string;
        account: string;
    }) {
        // TODO: get today
        const transaction = await invoke<Transaction>("create_expense", {
            amount,
            date,
            note,
            account
        });
        // this.#transactions.push(transaction);
        const transactions = this.transactions;
        this.#transactions = [transaction, ...transactions];
    }

    /**
     * Sets the outflow property of a transaction
     * @param id The id of the transaction
     * @param amount The amount to set as the outflow
     */
    async setOutflow({ id, amount }: { id: string; amount: string }) {
        const transaction = await invoke<Transaction>(
            "set_transaction_outflow",
            { id, amount }
        );
        const index = this.#transactions.findIndex(
            (t) => t.id === transaction.id
        );
        this.#transactions[index] = transaction;
    }

    async setPayee({ id, accountId }: { id: string; accountId: string }) {
        const transaction = await invoke<Transaction>(
            "set_transaction_payee",
            { id, accountId }
        );
        const index = this.#transactions.findIndex(
            (t) => t.id === transaction.id
        );
        this.#transactions[index] = transaction;
    }

    /**
     * Sets the inflow property of a transaction
     * @param id The id of the transaction
     * @param amount The amount to set as the outflow
     */
    async setInflow({ id, amount }: { id: string; amount: string }) {
        try {
            const transaction = await invoke<Transaction>(
                "set_transaction_inflow",
                { id, amount }
            );
            const index = this.#transactions.findIndex(
                (t) => t.id === transaction.id
            );
            this.#transactions[index] = transaction;
        } catch (e) {
            console.error(e);
        }
    }

    async editTransaction(opts: EditTransaction) {
        try {
            const transaction = await invoke<Transaction>("edit_transaction", {
                data: opts
            });
            const index = this.#transactions.findIndex(
                (t) => t.id === transaction.id
            );
            this.#transactions[index] = transaction;
            // TODO: re-sort because of date
            // this.#transactions
            //     .sort((a,b) => new SvelteDate(a.transactionDate).getTime()-new SvelteDate(b.transactionDate).getTime())
            //     .reverse();
        } catch (e) {
            const error = e as Error;
            logger.warn(`Failed to edit transaction: ${error.message}`);
        }
    }

    clear() {
        this.#transactions = [];
    }

    async load() {
        let transactions = await invoke<Transaction[]>("fetch_transactions");
        transactions
            .sort(
                (a, b) =>
                    new SvelteDate(a.transactionDate).getTime() -
                    new SvelteDate(b.transactionDate).getTime()
            )
            .reverse();
        this.#transactions = transactions;
    }
}

export const transactionStore = new TransactionStore();
