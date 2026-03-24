import { test, expect, afterEach } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { describe } from "node:test";
import { TransactionStore } from "$lib/stores/transaction.svelte";
import type { RawTransaction } from "$lib/transaction";
import { getLocalTimeZone, today } from "@internationalized/date";

afterEach(() => {
    clearMocks();
});

describe("TransactionStore",() => {
    test("loads transactions from IPC",async () => {
        mockIPC((cmd) => {
            if (cmd === "fetch_transactions" ) {
                let transactions: RawTransaction[] = [
                    { id:"1",fromAccountId:"A1",amount:"",transactionDate: today(getLocalTimeZone()).toString() },
                    { id:"2",fromAccountId:"A1",amount:"",transactionDate: today(getLocalTimeZone()).toString() },
                    { id:"3",fromAccountId:"A1",amount:"",transactionDate: today(getLocalTimeZone()).toString() }
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

