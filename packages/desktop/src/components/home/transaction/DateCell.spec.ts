import { test,beforeEach,expect, vi } from "vitest";
import { render } from "vitest-browser-svelte";
import type { RawTransaction, Transaction } from "$lib/api/transaction";
import { CalendarDate } from "@internationalized/date";
import { accountStore } from "$lib/stores/account.svelte";
import DateCell from "./DateCell.svelte";
import { transactionStore } from "$lib/stores/transaction.svelte";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { formatDate } from "$lib/utils/date";

beforeEach(() => {
    accountStore.clear();
    clearMocks();
});

test("Format date",async() => {
    mockIPC((cmd) => {
        if (cmd==="parse_date"){
            return "2023-12-12";
        }
        if(cmd === "edit_transaction"){
            const transaction: RawTransaction = {
                id: "1",
                amount: "20.00",
                transactionDate: "2020-10-10"
            };
            return transaction;
        }
    });
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
    mockIPC((cmd) => {
        if (cmd==="parse_date"){
            return "2023-12-12";
        }
        if(cmd === "edit_transaction"){
            const transaction: RawTransaction = {
                id: "1",
                amount: "20.00",
                transactionDate: "2020-10-10"
            };
            return transaction;
        }
    });
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
    mockIPC((cmd) => {
        if (cmd==="parse_date"){
            return "2023-12-12";
        }
        if(cmd === "edit_transaction"){
            const transaction: RawTransaction = {
                id: "1",
                amount: "20.00",
                transactionDate: "2020-10-10"
            };
            return transaction;
        }
    });
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
    // const date = day.element().getAttribute("data-value");
    await day.click();
    expect(spy).toHaveBeenCalledTimes(1);
});

test("Close calendar after selecting date",async() => {
    mockIPC((cmd) => {
        if (cmd==="parse_date"){
            return "2023-12-12";
        }
        if(cmd == "edit_transaction"){
            const transaction: RawTransaction = {
                id: "1",
                amount: "20.00",
                transactionDate: "2020-10-10"
            };
            return transaction;
        }
    });
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

