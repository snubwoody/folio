import { parseDate, type CalendarDate } from "@internationalized/date";
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
    // ??
    amount: number,
    fromAccountId?: string,
    toAccountId?: string,
    /** The date this transaction occurred. */
    date: CalendarDate,
    categoryId?: string,
    /** Optional context for the transaction */
    note?: string,
}

export async function getTransactions(): Promise<Transaction[]>{
    const rawTransactions = await invoke<RawTransaction[]>("fetch_transactions");
    const transactions =rawTransactions.map((value) => {
        const transaction: Transaction =  {
            id: value.id,
            amount: parseFloat(value.amount),
            fromAccountId: value.fromAccountId,
            toAccountId: value.toAccountId,
            note: value.note,
            categoryId: value.categoryId,
            date: parseDate(value.transactionDate)
        };
        return transaction;
    });
    return transactions;
}