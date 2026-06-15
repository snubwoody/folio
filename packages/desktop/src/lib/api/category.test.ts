import { getLocalTimeZone, today } from "@internationalized/date";
import { describe, expect, test } from "vitest";
import { totalSpent } from "./category";
import type { Transaction } from "./transaction";

describe("totalSpent", () => {
    test("total in current month", () => {
        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: "C1",
                date: today(getLocalTimeZone()),
                amount: "100",
            },
            {
                id: "A1",
                categoryId: "C1",
                date: today(getLocalTimeZone()),
                amount: "200",
            },
        ];
        const total = totalSpent("C1", transactions, today(getLocalTimeZone()));
        expect(total).toBe(300);
    });
    test("only include matching categories", () => {
        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: "C1",
                date: today(getLocalTimeZone()),
                amount: "100",
            },
            {
                id: "A1",
                categoryId: "Ciojoij2",
                date: today(getLocalTimeZone()),
                amount: "200",
            },
        ];
        const total = totalSpent("C1", transactions, today(getLocalTimeZone()));
        expect(total).toBe(100);
    });
});
