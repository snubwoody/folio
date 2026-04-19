// Copyright (C) 2025 Wakunguma Kalimukwa
// SPDX-License-Identifier: GPL-3.0-or-later

// Wrappers for interacting with transactions from the backend.
import { CalendarDate, getLocalTimeZone, now, parseDate, toCalendarDate } from "@internationalized/date";
import { invoke } from "@tauri-apps/api/core";
import { mockIPC } from "@tauri-apps/api/mocks";

export type TransactionType = "Expense" | "Income" | "Transfer";

/**
 * Raw transaction from the backend
 */
export interface RawTransaction {
    id: string;
    amount: string;
    fromAccountId?: string;
    toAccountId?: string;
    transactionDate: string;
    categoryId?: string;
    note?: string;
}

export interface Transaction{
    id: string,
    amount: string,
    fromAccountId?: string,
    toAccountId?: string,
    /** The date this transaction occurred. */
    date: CalendarDate,
    categoryId?: string,
    /** Optional context for the transaction */
    note?: string,
}

export interface CreateTransactionOpts{
    /** The account id. */
    accountId: string
    /** The amount exchanged, defaults to 0. */
    amount?: string,
    /** The date the transaction occurred. */
    date?: CalendarDate,
}

export interface EditTransactionOpts {
    id: string;
    fromAccountId?: string;
    note?: string;
    categoryId?: string;
    transactionDate?: string;
    amount?: string;
    toAccountId?: string;
}

/**
 * Parses a raw transaction from the backend into the user facing.
 * {@link Transaction} type.
 * @param value The raw transaction
 * @returns The parsed transaction
 */
function parseTransaction(value: RawTransaction): Transaction {
    return {
        id: value.id,
        amount: value.amount,
        fromAccountId: value.fromAccountId,
        toAccountId: value.toAccountId,
        note: value.note,
        categoryId: value.categoryId,
        date: parseDate(value.transactionDate)
    };
}

/**
 * Fetches all the transactions from the database.
 */
export async function getTransactions(): Promise<Transaction[]>{
    const rawTransactions = await invoke<RawTransaction[]>("fetch_transactions");
    return rawTransactions.map((value) => parseTransaction(value));
}

/**
 * Creates a new transaction.
 */
export async function createTransaction({
    accountId,
    amount = "0.00",
    date = toCalendarDate(now(getLocalTimeZone()))
}:CreateTransactionOpts): Promise<Transaction>{
    const response = await invoke<RawTransaction>("create_expense", {
        amount,
        date: date.toString(),
        account: accountId
    });

    return parseTransaction(response);
}

/**
 * Sets the outflow property of a transaction. Calling this on an income
 * will convert it into an expense.
 *
 * @param id The id of the transaction
 * @param amount The amount to set as the outflow
 */
export async function setOutflow(id:string,amount:string): Promise<Transaction> {
    const transaction = await invoke<RawTransaction>(
        "set_transaction_outflow",
        { id, amount }
    );
    return parseTransaction(transaction);
}

/**
 * Sets the account property of a transaction.
 *
 * @param id The id of the transaction
 * @param account The id of the account
 */
export async function setAccount(id:string,account:string): Promise<Transaction> {
    const transaction = await invoke<RawTransaction>(
        "set_transaction_account",
        { id, account }
    );
    return parseTransaction(transaction);
}

/**
 * Sets the inflow property of a transaction. Calling this on an expense
 * will convert it into an income.
 * @param id The id of the transaction
 * @param amount The amount to set as the outflow
 */
export async function setInflow(id:string,amount:string): Promise<Transaction> {
    const transaction = await invoke<RawTransaction>(
        "set_transaction_inflow",
        { id, amount }
    );
    return parseTransaction(transaction);
}

/**
 * Sets the payee of a transaction.
 * @param id The id of the transaction
 * @param accountId The id of the payee account
 */
export async function setPayee(id:string,accountId:string): Promise<Transaction> {
    const transaction = await invoke<RawTransaction>(
        "set_transaction_payee",
        { id, accountId }
    );
    return parseTransaction(transaction);
}

export async function editTransaction(opts: EditTransactionOpts) {
    const transaction = await invoke<RawTransaction>("edit_transaction", {
        data: opts
    });

    return parseTransaction(transaction);
}

/**
 * Deletes a list of transactions.
 * @param ids The ids of the transactions to delete
 */
export async function deleteTransactions(ids: string[]) {
    await invoke("delete_transactions", { ids });
}

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