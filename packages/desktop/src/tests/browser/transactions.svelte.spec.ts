import { getLocalTimeZone, parseDate, today } from "@internationalized/date";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { afterEach, beforeEach, describe, expect, test } from "vitest";
import { render } from "vitest-browser-svelte";
import Actionbar from "$components/home/transaction/Actionbar.svelte";
import Toolbar from "$components/home/transaction/Toolbar.svelte";
import TransactionComponent from "$components/home/transaction/Transaction.svelte";
import type { RawTransaction, Transaction } from "$lib/api/transaction";
import { accountStore } from "$lib/stores/account.svelte";
import { TableStore } from "$lib/stores/table.svelte";
import { randomId } from "$lib/stores/toast.svelte";
import { transactionStore } from "$lib/stores/transaction.svelte";

beforeEach(() => {
    accountStore.clear();
    transactionStore.clear();
});

afterEach(() => {
    clearMocks();
});

describe("Transaction toolbar", () => {
    test("contains add transaction button", async () => {
        const screen = render(Toolbar);
        expect(
            screen.getByRole("button", { name: "Add Transaction" }),
        ).toBeInTheDocument();
    });
    test("add transaction button creates a new transaction", async () => {
        mockIPC((cmd) => {
            const transaction: RawTransaction = {
                id: "t1",
                amount: "",
                transactionDate: today(getLocalTimeZone()).toString(),
            };

            if (cmd === "create_expense") {
                return transaction;
            }
        });
        await accountStore.createTestAccount({ name: "" });
        const screen = render(Toolbar);
        expect(transactionStore.transactions).toHaveLength(0);
        await screen.getByRole("button", { name: "Add Transaction" }).click();
        expect(transactionStore.transactions).toHaveLength(1);
        expect(transactionStore.transactions[0].id).toBe("t1");
    });
});

describe("Transaction actionbar", () => {
    test("show single transaction", async () => {
        const tableStore = new TableStore();
        tableStore.select("1");
        const screen = render(Actionbar, { tableStore });
        expect(screen.getByText("1 transaction")).toBeVisible();
    });
    test("show multiple transactions", async () => {
        const tableStore = new TableStore();
        tableStore.select("1");
        tableStore.select("2");
        tableStore.select("3");
        const screen = render(Actionbar, { tableStore });
        expect(screen.getByText("3 transactions")).toBeVisible();
    });
    test("show all transactions", async () => {
        mockIPC((cmd) => {
            if (cmd === "create_expense") {
                const t1: RawTransaction = {
                    id: randomId(),
                    transactionDate: today(getLocalTimeZone()).toString(),
                    amount: "",
                };
                return t1;
            }
        });
        await transactionStore.createExpense({
            amount: "0.00",
            date: today(getLocalTimeZone()),
            accountId: "",
        });
        await transactionStore.createExpense({
            amount: "0.00",
            date: today(getLocalTimeZone()),
            accountId: "",
        });
        await transactionStore.createExpense({
            amount: "0.00",
            date: today(getLocalTimeZone()),
            accountId: "",
        });
        await transactionStore.createExpense({
            amount: "0.00",
            date: today(getLocalTimeZone()),
            accountId: "",
        });

        const tableStore = new TableStore();
        tableStore.toggleSelectAll();
        const screen = render(Actionbar, { tableStore });
        expect(screen.getByText("4 transactions")).toBeVisible();
    });
    test("show delete button", async () => {
        const tableStore = new TableStore();
        tableStore.select("1");
        const screen = render(Actionbar, { tableStore });
        expect(screen.getByRole("button", { name: "Delete" })).toBeVisible();
    });
    test("show close button", async () => {
        const tableStore = new TableStore();
        tableStore.select("1");
        const screen = render(Actionbar, { tableStore });
        expect(screen.getByRole("button", { name: "Close" })).toBeVisible();
    });
    test("close button closes action bar", async () => {
        const tableStore = new TableStore();
        tableStore.select("1");
        const screen = render(Actionbar, { tableStore });
        await screen.getByRole("button", { name: "Close" }).click();
        expect(tableStore.selectedRows.size).toBe(0);
        expect(tableStore.allRowsSelected).toBe(false);
    });
    test("delete button deletes transactions", async () => {
        mockIPC((cmd) => {
            if (cmd === "create_expense") {
                const t1: RawTransaction = {
                    id: "t1",
                    transactionDate: today(getLocalTimeZone()).toString(),
                    amount: "",
                };
                return t1;
            }
        });

        await transactionStore.createExpense({
            amount: "",
            date: today(getLocalTimeZone()),
            accountId: "",
        });
        expect(transactionStore.transactions).toHaveLength(1);
        const tableStore = new TableStore();
        tableStore.select("t1");
        const screen = render(Actionbar, { tableStore });
        await screen.getByRole("button", { name: "Delete" }).click();
        expect(transactionStore.transactions).toHaveLength(0);
    });
});

// TODO: test popover
describe("Transaction component", async () => {
    test("shows outflow if transaction is an expense", async () => {
        const transaction: Transaction = {
            id: "1",
            fromAccountId: "A1",
            amount: "500.0",
            date: parseDate("2024-12-12"),
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent, {
            transaction,
            tableStore,
        });
        const outflow = screen.getByTestId("outflow");
        const inflow = screen.getByTestId("inflow");
        expect(outflow).toHaveTextContent("K500.00");
        expect(inflow).toHaveTextContent("Select an item");
    });
    test("shows inflow if transaction is an income", async () => {
        const transaction: Transaction = {
            id: "1",
            toAccountId: "A1",
            amount: "500.0",
            date: parseDate("2024-12-12"),
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent, {
            transaction,
            tableStore,
        });
        const outflow = screen.getByTestId("outflow");
        const inflow = screen.getByTestId("inflow");
        expect(outflow).toHaveTextContent("Select an item");
        expect(inflow).toHaveTextContent("K500.00");
    });
    test("shows account if expense", async () => {
        const account = await accountStore.createTestAccount({
            name: "Account 1",
        });

        const transaction: Transaction = {
            id: "1",
            fromAccountId: account.id,
            amount: "500.0",
            date: parseDate("2024-12-12"),
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent, {
            transaction,
            tableStore,
        });
        const accountCell = screen.getByTestId("account");
        expect(accountCell).toHaveTextContent("Account 1");
    });
    test("shows account if income", async () => {
        const account = await accountStore.createTestAccount({
            name: "Account 1",
        });

        const transaction: Transaction = {
            id: "1",
            toAccountId: account.id,
            amount: "500.0",
            date: parseDate("2024-12-12"),
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent, {
            transaction,
            tableStore,
        });
        const accountCell = screen.getByTestId("account");
        expect(accountCell).toHaveTextContent("Account 1");
    });
});
