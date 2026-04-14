import { test, expect, afterEach,describe } from "vitest";
import {createTransaction, getTransactions, type RawTransaction, type Transaction} from "./transaction";
import {getLocalTimeZone, now, parseDate, toCalendarDate, today} from "@internationalized/date";
import {totalSpent} from "./category";
import {transactionStore} from "$lib/stores/transaction.svelte";

describe("totalSpent",()=>{
    test("total in current month",()=>{
        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: "C1",
                date: today(getLocalTimeZone()),
                amount: "100"
            },
            {
                id: "A1",
                categoryId: "C1",
                date: today(getLocalTimeZone()),
                amount: "200"
            },
        ];
        const total = totalSpent("C1",transactions,today(getLocalTimeZone()));
        expect(total).toBe(300);
    });
    test("only include matching categories",()=>{
        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: "C1",
                date: today(getLocalTimeZone()),
                amount: "100"
            },
            {
                id: "A1",
                categoryId: "Ciojoij2",
                date: today(getLocalTimeZone()),
                amount: "200"
            },
        ];
        const total = totalSpent("C1",transactions,today(getLocalTimeZone()));
        expect(total).toBe(100);
    });
});