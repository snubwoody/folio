import { test, expect, afterEach } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { describe } from "node:test";
import { type Transaction,TransactionStore } from "$lib/transaction.svelte";

afterEach(() => {
    clearMocks();
});

describe("TransactionStore",()=>{
    test("loads transactions from IPC",async ()=>{
        mockIPC((cmd) => {
            if (cmd === "fetch_transactions" ) {
                let transactions: Transaction[] = [
                    {id:"1",from_account_id:"A1",transaction_date:"",amount:""},
                    {id:"2",from_account_id:"A1",transaction_date:"",amount:""},
                    {id:"3",from_account_id:"A1",transaction_date:"",amount:""},
                ];
                return transactions;
            }
        });

        const transactionStore = new TransactionStore();
        await transactionStore.load();
        expect(transactionStore.transactions).toHaveLength(3);
        expect(transactionStore.transactions[1].id).toBe("2");
    });
});


