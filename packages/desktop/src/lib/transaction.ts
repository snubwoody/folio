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

interface CreateTransactionOpts{
    /** The account id. */
    accountId: string
    /** The amount exchanged, defaults to 0. */
    amount?: string,
    /** The date the transaction occurred. */
    date?: CalendarDate,
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
        date: date.toString(),
        account: accountId
    });

    return parseTransaction(response);
}

/**
 * Sets the outflow property of a transaction. Calling this on an income
 * will convert it into an expense.
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
 * Deletes a list of transactions.
 * @param ids The ids of the transactions to delete
 */
export async function deleteTransactions(ids: string[]) {
    await invoke("delete_transactions", { ids });
}