import { test, expect, afterEach } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { describe } from "node:test";
import { type Transaction,TransactionStore } from "$lib/stores/transaction.svelte";
import {createTransaction, getTransactions} from "./transaction";
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
    expect(transaction.amount).toBe("24.24");
    expect(transaction.id).toBe("1");
    expect(transaction.fromAccountId).toBe("A1");
    expect(transaction.toAccountId).toBe("A2");
    expect(transaction.categoryId).toBe("C1");
    expect(transaction.note).toBe("Note");
});

test("createTransaction",async()=>{
    mockIPC((cmd,args) => {
        if (cmd === "create_expense" ) {
            console.log(args);
            const payload = args as {amount:string,date:string,account:string};
            let transactions: Transaction = {
                id:"1",
                fromAccountId:payload.account,
                transactionDate:payload.date,
                amount:payload.amount, 
            };
            return transactions;
        }
    });

    const transaction = await createTransaction({
        accountId:"A1",
        amount:"120",
        date: parseDate("2024-12-12")
    });

    expect(transaction.date).toBe(parseDate("2024-12-12"));
    expect(transaction.fromAccountId).toBe("A1");
    expect(transaction.amount).toBe("120");
});


