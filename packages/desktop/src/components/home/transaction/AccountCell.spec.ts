import {test,beforeEach,expect,describe} from "vitest";
import AccountCell from "./AccountCell.svelte";
import { render } from "vitest-browser-svelte";
import type { Transaction } from "$lib/transaction";
import { CalendarDate, parseDate } from "@internationalized/date";
import { accountStore } from "$lib/stores/account.svelte";

beforeEach(() => {
    accountStore.clear();
});

// TODO: select account
test("Exclude payee from select menu",async()=>{
    const a1 = await accountStore.createTestAccount({name:"Account 1"});
    const a2 = await accountStore.createTestAccount({name:"Payee"});
    const a3 = await accountStore.createTestAccount({name:"Account 2"});
    const transaction: Transaction = {
        id: "1",
        amount: "10.00",
        fromAccountId: a1.id,
        toAccountId: a2.id,
        date: new CalendarDate(2020,1,1)
    };
    const screen = await render(AccountCell,{transaction});
    await screen.getByRole("button").click();
    // await expect.element(screen.getByText("Savings")).toBeInTheDocument();
});

test("Display expense account",async()=>{
    const account = await accountStore.createTestAccount({name:"Savings"});
    const transaction: Transaction = {
        id: "1",
        amount: "10.00",
        fromAccountId: account.id,
        date: new CalendarDate(2020,1,1)
    };
    const screen = await render(AccountCell,{transaction});
    await expect.element(screen.getByText("Savings")).toBeInTheDocument();
});

test("Display expense account", async () => {
    const account = await accountStore.createTestAccount({ name: "Income account" });

    const transaction: Transaction = {
        id: "1",
        toAccountId: account.id,
        amount: "500.0",
        date: parseDate("2024-12-12")
    };

    const screen = render(AccountCell, {
        transaction
    });
    const accountCell = screen.getByTestId("account");
    expect(accountCell).toHaveTextContent("Income account");
});

test("Display expense account if transfer", async () => {
    const account = await accountStore.createTestAccount({ name: "Income account" });
    const account2 = await accountStore.createTestAccount({ name: "Expense account" });

    const transaction: Transaction = {
        id: "1",
        toAccountId: account.id,
        fromAccountId: account2.id,
        amount: "500.0",
        date: parseDate("2024-12-12")
    };

    const screen = render(AccountCell, {
        transaction
    });
    const accountCell = screen.getByTestId("account");
    expect(accountCell).toHaveTextContent("Expense account");
});
