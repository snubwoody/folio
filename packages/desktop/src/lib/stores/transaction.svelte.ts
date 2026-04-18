// Copyright (C) 2025 Wakunguma Kalimukwa
// SPDX-License-Identifier: GPL-3.0-or-later
import { logger } from "../utils/logger";
import {
    getTransactions,
    type Transaction,
    type EditTransactionOpts,
    deleteTransactions,
    editTransaction,
    setInflow,
    setOutflow,
    setPayee,
    createTransaction,
    type CreateTransactionOpts,
    setAccount
} from "$lib/api/transaction";

export class TransactionStore {
    #transactions: Transaction[] = $state([]);

    get transactions() {
        return this.#transactions;
    }

    addTestTransaction(transaction: Transaction){
        this.#transactions.push(transaction);
    }

    /**
     * Deletes a list of transactions.
     * @param ids The ids of the transactions to delete.
     */
    async deleteTransactions(ids: string[]){
        await deleteTransactions(ids);
        this.#transactions = this.#transactions.filter(t => !ids.includes(t.id));
    }

    /**
     * Creates a new expense.
     *
     * @param opts Options used for creating the expense.
     */
    async createExpense(opts: CreateTransactionOpts) {
        const transaction = await createTransaction(opts);
        const transactions = this.transactions;
        this.#transactions = [transaction, ...transactions];
    }

    /**
     * Sets the outflow property of a transaction.
     * @param id The id of the transaction
     * @param amount The amount to set as the outflow
     */
    async setOutflow(id: string, amount: string) {
        const transaction = await setOutflow(id,amount);
        const index = this.#transactions.findIndex(
            (t) => t.id === transaction.id
        );
        this.#transactions[index] = transaction;
    }

    /**
     * Sets the account property of a transaction.
     * @param id The id of the transaction
     * @param account The id of the account
     */
    async setAccount(id: string, account: string) {
        const transaction = await setAccount(id,account);
        const index = this.#transactions.findIndex(
            (t) => t.id === transaction.id
        );
        this.#transactions[index] = transaction;
    }

    /**
     * Sets the payee of a transaction.
    * @param id The id of the transaction
    * @param accountId The id of the payee account
    */
    async setPayee(id:string, accountId:string) {
        const transaction = await setPayee(id,accountId);
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
    async setInflow(id:string,amount:string) {
        const transaction = await setInflow(id,amount);
        const index = this.#transactions.findIndex((t) => t.id === transaction.id);
        this.#transactions[index] = transaction;
    }

    async editTransaction(opts: EditTransactionOpts) {
        const transaction = await editTransaction(opts);
        const index = this.#transactions.findIndex((t) => t.id === transaction.id);
        this.#transactions[index] = transaction;

    }

    /**
     * Sorts the transactions in place.
     */
    sort(){
        this.#transactions
            .sort((a, b) => a.date.compare(b.date))
            .reverse();
    }

    /**
     * Empties the transaction list.
     */
    clear() {
        this.#transactions = [];
    }

    /**
     * Loads the transactions from the backend.
     */
    async load() {
        let transactions = await getTransactions();
        transactions
            .sort((a, b) => a.date.compare(b.date))
            .reverse();
        this.#transactions = transactions;
        logger.debug("Loaded transactions from backend");
    }
}

export const transactionStore = new TransactionStore();
