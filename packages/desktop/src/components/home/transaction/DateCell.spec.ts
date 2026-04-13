import { test,beforeEach,expect, vi } from "vitest";
import { render } from "vitest-browser-svelte";
import { mockTransactions, type Transaction } from "$lib/api/transaction";
import { CalendarDate } from "@internationalized/date";
import { accountStore } from "$lib/stores/account.svelte";
import DateCell from "./DateCell.svelte";
import { transactionStore } from "$lib/stores/transaction.svelte";
import { clearMocks } from "@tauri-apps/api/mocks";
import {formatDate} from "$lib/utils/date";

beforeEach(() => {
    accountStore.clear();
    clearMocks();
});

test("Format date",async() => {
    const transaction: Transaction = {
        id: "1",
        amount: "10.00",
        fromAccountId: "",
        toAccountId: "",
        date: new CalendarDate(2020,1,1)
    };
    const screen = await render(DateCell,{ transaction });
    await expect.element(screen.getByRole("button",{ name: formatDate(transaction.date) })).toBeInTheDocument();
});

test("Open calendar",async() => {
    const transaction: Transaction = {
        id: "1",
        amount: "10.00",
        fromAccountId: "",
        toAccountId: "",
        date: new CalendarDate(2020,1,1)
    };
    const screen = await render(DateCell,{ transaction });
    await screen.getByRole("button",{ name: formatDate(transaction.date) }).click();
    const calendar = screen.getByTestId("calendar");
    await expect.element(calendar).toBeInTheDocument();
});

test("Change date",async() => {
    mockTransactions();
    const spy = vi.spyOn(transactionStore,"editTransaction");
    const transaction: Transaction = {
        id: "1",
        amount: "10.00",
        fromAccountId: "",
        toAccountId: "",
        date: new CalendarDate(2020,1,1)
    };
    const screen = await render(DateCell,{ transaction });
    await screen.getByRole("button",{ name: formatDate(transaction.date) }).click();
    const calendar = screen.getByTestId("calendar");
    const days = calendar.getByRole("gridcell").all();
    const day = days[Math.round(days.length/2)];
    const date = day.element().getAttribute("data-value");
    await day.click();
    expect(spy).toHaveBeenCalledWith({
        id:"1",
        transactionDate: date
    });
});

test("Close calendar after selecting date",async() => {
    mockTransactions();
    const transaction: Transaction = {
        id: "1",
        amount: "10.00",
        fromAccountId: "",
        toAccountId: "",
        date: new CalendarDate(2020,1,1)
    };
    const screen = await render(DateCell,{ transaction });
    await screen.getByRole("button",{ name: formatDate(transaction.date) }).click();
    const calendar = screen.getByTestId("calendar");
    await expect.element(calendar).toBeInTheDocument();
    const days = calendar.getByRole("gridcell").all();
    const day = days[Math.round(days.length/2)];
    await day.click();
    await expect.element(calendar).not.toBeInTheDocument();
});

