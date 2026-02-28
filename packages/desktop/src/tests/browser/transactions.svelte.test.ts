import { test, beforeEach, describe, expect, afterEach } from "vitest";
import TransactionComponent from "$components/home/transaction/Transaction.svelte";
import Toolbar from "$components/home/transaction/Toolbar.svelte";
import Actionbar from "$components/home/transaction/Actionbar.svelte";
import { transactionStore, type Transaction } from "$lib/transaction.svelte";
import { render } from "vitest-browser-svelte";
import { TableStore } from "$lib/stores/table.svelte";
import { accountStore } from "$lib/account.svelte";

import { mockIPC, clearMocks } from "@tauri-apps/api/mocks";
import { randomId } from "$lib/toast.svelte";

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
            screen.getByRole("button", { name: "Add Transaction" })
        ).toBeInTheDocument();
    });
    test("add transaction button creates a new transaction", async () => {
        mockIPC((cmd) => {
            const transaction: Transaction = {
                id: "t1",
                amount: "",
                transactionDate: ""
            };

            if (cmd == "create_expense") {
                return transaction;
            }
        });
        await accountStore.createAccount({ name: "" });
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
                const t1: Transaction = { id: randomId(),transactionDate:"",amount:"" };
                return t1;
            }
        });
        await transactionStore.createExpense({ amount: "", date: "", account: "" });
        await transactionStore.createExpense({ amount: "", date: "", account: "" });
        await transactionStore.createExpense({ amount: "", date: "", account: "" });
        await transactionStore.createExpense({ amount: "", date: "", account: "" });

        const tableStore = new TableStore();
        tableStore.toggleSelectAll();
        const screen = render(Actionbar, { tableStore });
        expect(screen.getByText("4 transactions")).toBeVisible();
    });
    test("show delete button", async () => {
        const tableStore = new TableStore();
        tableStore.select("1");
        const screen = render(Actionbar, { tableStore });
        expect(screen.getByRole("button",{ name:"Delete" })).toBeVisible();
    });
    test("show close button", async () => {
        const tableStore = new TableStore();
        tableStore.select("1");
        const screen = render(Actionbar, { tableStore });
        expect(screen.getByRole("button",{ name:"Close" })).toBeVisible();
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
                const t1: Transaction = { id: "t1",transactionDate:"",amount:"" };
                return t1;
            }
        });

        await transactionStore.createExpense({ amount: "", date:"",account:"" });
        expect(transactionStore.transactions).toHaveLength(1);
        const tableStore = new TableStore();
        tableStore.select("t1");
        const screen = render(Actionbar, { tableStore });
        await screen.getByRole("button", { name: "Delete" }).click();
        expect(transactionStore.transactions).toHaveLength(0);
    });
});

// TODO: test
// - select
// - category
describe("Transaction component", async () => {
    test("shows outflow if transaction is an expense", async () => {
        const transaction: Transaction = {
            id: "1",
            fromAccountId: "A1",
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent, {
            transaction,
            tableStore
        });
        const outflow = screen.getByTestId("outflow");
        const inflow = screen.getByTestId("inflow");
        expect(outflow.getByRole("paragraph")).toHaveTextContent("$");
        expect(outflow.getByRole("textbox")).toHaveValue("500.00");
        expect(inflow.getByRole("textbox")).toHaveValue("");
    });
    test("shows inflow if transaction is an income", async () => {
        // TODO: test edit
        const transaction: Transaction = {
            id: "1",
            toAccountId: "A1",
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent, {
            transaction,
            tableStore
        });
        const outflow = screen.getByTestId("outflow");
        const inflow = screen.getByTestId("inflow");
        expect(outflow.getByRole("textbox")).toHaveValue("");
        expect(inflow).toHaveTextContent("$");
        expect(inflow.getByRole("textbox")).toHaveValue("500.00");
    });
    test("shows account if expense", async () => {
        const account = await accountStore.createAccount({ name: "Account 1" });

        const transaction: Transaction = {
            id: "1",
            fromAccountId: account.id,
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        console.log(transaction.fromAccountId);

        const screen = render(TransactionComponent, {
            transaction,
            tableStore
        });
        const accountCell = screen.getByTestId("account");
        const payeeCell = screen.getByTestId("payee");
        expect(accountCell).toHaveTextContent("Account 1");
        expect(payeeCell).toBeInTheDocument();
        expect(payeeCell.query()?.children).toHaveLength(0);
    });
    test("shows account if income", async () => {
        const account = await accountStore.createAccount({ name: "Account 1" });

        const transaction: Transaction = {
            id: "1",
            toAccountId: account.id,
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        console.log(transaction.fromAccountId);

        const screen = render(TransactionComponent, {
            transaction,
            tableStore
        });
        const accountCell = screen.getByTestId("account");
        const payeeCell = screen.getByTestId("payee");
        expect(accountCell).toHaveTextContent("Account 1");
        expect(payeeCell).toBeInTheDocument();
        expect(payeeCell.query()?.children).toHaveLength(0);
    });
});
