import { CalendarDate, getLocalTimeZone, now, parseDate, toCalendarDate } from "@internationalized/date";
import { invoke } from "@tauri-apps/api/core";



/**
 * Raw transaction from the backend
 */
interface RawTransaction {
    id: string;
    amount: string;
    fromAccountId?: string;
    toAccountId?: string;
    transactionDate: string;
    categoryId?: string;
    note?: string;
}

interface Transaction{
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

/**
 * Parses a raw transaction from the backend into the user facing 
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
    const transactions = rawTransactions.map((value) => {
        const transaction = parseTransaction(value);
        return transaction;
    });
    return transactions;
}

interface CreateTransactionOpts{
    /** The account id. */
    accountId: string
    /** The amount exchanged, defaults to 0. */
    amount?: string,
    /** The date the transaction occurred. */
    date?: CalendarDate,
}

/** 
 * Creates a new transaction
 */
export async function createTransaction({
    accountId,
    amount = "0.00",
    date = toCalendarDate(now(getLocalTimeZone()))
}:CreateTransactionOpts): Promise<Transaction>{
    const response = await invoke<RawTransaction>("create_expense", {
        amount,
        date,
        account: accountId
    });

    return parseTransaction(response);
}

    // async createExpense({
    //     amount = "0.0",
    //     date,
    //     note,
    //     account
    // }: {
    //     amount: string;
    //     date: string;
    //     note?: string;
    //     account: string;
    // }) {
    //     // TODO: set current date as default
    //     const transaction = await invoke<Transaction>("create_expense", {
    //         amount,
    //         date,
    //         note,
    //         account
    //     });
    //     const transactions = this.transactions;
    //     this.#transactions = [transaction, ...transactions];
    // }


async function deleteTransactions(ids: string[]) {
    // TODO: resort
    await invoke("delete_transactions", { ids });
}