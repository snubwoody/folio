import { test, beforeEach,describe, expect } from "vitest";
import TransactionComponent from "$components/home/transaction/Transaction.svelte";
import type { Transaction } from "$lib/transaction.svelte";
import { render } from "vitest-browser-svelte";
import { TableStore } from "$lib/stores/table.svelte";
import { accountStore } from "$lib/account.svelte";

beforeEach(() => {
    accountStore.clear();
});

// TODO: test
// - select
// - income
// - expense
// - transfer
// - category
describe("Transaction component",async() => {
    test("shows outflow if transaction is an expense",async() => {
        const transaction: Transaction ={
            id: "1",
            fromAccountId: "A1",
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent,{ transaction,tableStore });
        const outflow = screen.getByTestId("outflow");
        const inflow = screen.getByTestId("inflow");
        expect(outflow.getByRole("paragraph")).toHaveTextContent("$");
        expect(outflow.getByRole("textbox")).toHaveValue("500.00");
        expect(inflow.getByRole("textbox")).toHaveValue("");
    });
    test("shows inflow if transaction is an income",async() => {
        // TODO: test edit
        const transaction: Transaction ={
            id: "1",
            toAccountId: "A1",
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent,{ transaction,tableStore });
        const outflow = screen.getByTestId("outflow");
        const inflow = screen.getByTestId("inflow");
        expect(outflow.getByRole("textbox")).toHaveValue("");
        expect(inflow).toHaveTextContent("$");
        expect(inflow.getByRole("textbox")).toHaveValue("500.00");
    });
    test("shows account if expense",async() => {
        const account = await accountStore.createAccount({ name:"Account 1" });

        const transaction: Transaction ={
            id: "1",
            fromAccountId: account.id,
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        console.log(transaction.fromAccountId);

        const screen = render(TransactionComponent,{ transaction,tableStore });
        const accountCell = screen.getByTestId("account");
        const payeeCell = screen.getByTestId("payee");
        expect(accountCell).toHaveTextContent("Account 1");
        expect(payeeCell).toBeInTheDocument();
        expect(payeeCell.query()?.children).toHaveLength(0);
    });
    test("shows account if income",async() => {
        const account = await accountStore.createAccount({ name:"Account 1" });

        const transaction: Transaction ={
            id: "1",
            toAccountId: account.id,
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        console.log(transaction.fromAccountId);

        const screen = render(TransactionComponent,{ transaction,tableStore });
        const accountCell = screen.getByTestId("account");
        const payeeCell = screen.getByTestId("payee");
        expect(accountCell).toHaveTextContent("Account 1");
        expect(payeeCell).toBeInTheDocument();
        expect(payeeCell.query()?.children).toHaveLength(0);
    });
});

