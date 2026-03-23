import { test, expect, afterEach } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { describe } from "node:test";
import { type Transaction,TransactionStore } from "$lib/stores/transaction.svelte";
import {getTransactions} from "./transaction";
import { parseDate } from "@internationalized/date";

afterEach(() => {
    clearMocks();
});

test("getTransactions",async()=>{
    mockIPC((cmd) => {
        if (cmd === "fetch_transactions" ) {
            let transactions: Transaction[] = [
                {
                    id:"1",
                    fromAccountId:"A1",
                    toAccountId:"A2",
                    transactionDate:"2023-12-01",
                    amount:"24.24", 
                    note: "Note",
                    categoryId: "C1",
                },
            ];
            return transactions;
        }
    });

    const transactions = await getTransactions();
    const transaction = transactions[0];
    expect(transaction.date).toStrictEqual(parseDate("2023-12-01"));
    expect(transaction.amount).toBe(24.24);
    expect(transaction.id).toBe("1");
    expect(transaction.fromAccountId).toBe("A1");
    expect(transaction.toAccountId).toBe("A2");
    expect(transaction.categoryId).toBe("C1");
    expect(transaction.note).toBe("Note");
});


