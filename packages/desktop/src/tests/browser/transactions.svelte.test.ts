import { test, beforeEach,describe, expect } from "vitest";
import TransactionComponent from "$components/home/Transaction.svelte";
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
describe("Transction component",async() => {
    test("shows outflow if transaction is an expense",async() => {
        const transaction: Transaction ={
            id: "1",
            fromAccountId: "A1",
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        const screen = render(TransactionComponent,{ transaction,tableStore });
        const outflow = screen.getByRole("cell").nth(7);
        const inflow = screen.getByRole("cell").nth(8);
        expect(outflow).toHaveTextContent("$500.00");
        expect(inflow).toBeEmptyDOMElement();
    });
    test("shows inflow if transaction is an income",async() => {
        const transaction: Transaction ={
            id: "1",
            toAccountId: "A1",
            amount: "500.0",
            transactionDate: "2024-12-12"
        };

        const tableStore = new TableStore();

        console.log(transaction.fromAccountId);

        const screen = render(TransactionComponent,{ transaction,tableStore });
        const outflow = screen.getByRole("cell").nth(7);
        const inflow = screen.getByRole("cell").nth(8);
        console.log(outflow);
        console.log(inflow);
        expect(outflow).toBeEmptyDOMElement();
        expect(inflow).toHaveTextContent("$500.00");
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

// test("Expense table heading", async () => {
//     // const page = render(ExpenseTable);
//     // expect(page.getByText("Category")).toBeInTheDocument();
//     // expect(page.getByText("Account")).toBeInTheDocument();
//     // expect(page.getByText("Date")).toBeInTheDocument();
//     // expect(page.getByText("Amount")).toBeInTheDocument();
// });

// test("Show add transaction button", async () => {
//     // const page = render(TransactionsSection);
//     // expect(page.getByRole("button", { name: "New" })).toBeInTheDocument();
// });

// test("Open add expense form", async () => {
//     // const page = render(TransactionsSection);
//     // await page.getByRole("button", { name: "New" }).click();
//     // const fields = page.getByRole("textbox").all();
//     // expect(fields).toHaveLength(2);
// });

// test("Show expenses in expense table", async () => {
//     // appStore.settings.currencyCode = "CAD";
//     // const account: Account= {
//     //     id: "24",
//     //     startingBalance: "24",
//     //     balance: "24",
//     //     name: "Account"
//     // };
//     // appStore.accounts = [account];
//     // appStore.expenses = [
//     //     { id: "1", amount: "0",date: "2025-10-11",currencyCode: "CAD", account },
//     //     { id: "2", amount: "500",date: "2025-10-10",currencyCode: "CAD",account },
//     //     { id: "3", amount: "24.24",date: "2025-09-01",currencyCode: "CAD",account }
//     // ];
//     // const page = render(ExpenseTable);
//     // const table = page.getByRole("table");
//     // expect(table.getByText("Oct 10, 2025")).toBeInTheDocument();
//     // expect(table.getByText("CA$").first()).toBeInTheDocument();
// });

// test("Show expense category", async () => {
//     // appStore.settings.currencyCode = "CAD";
//     // const category: Category = {
//     //     id: "24",
//     //     title: "Transport",
//     //     createdAt: "2025-01-01"
//     // };
//     // const expense: Expense = {
//     //     id: "1",
//     //     amount: "22.24",
//     //     date: "2024-09-09",
//     //     category,
//     //     currencyCode: "USD"
//     // };
//     // appStore.categories = [category];
//     // appStore.expenses = [expense];
//     // const page = render(ExpenseTable);
//     // expect(page.getByText("Transport")).toBeInTheDocument();
// });
