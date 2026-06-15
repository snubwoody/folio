import { beforeEach, describe, expect, test } from "vitest";
import { render } from "vitest-browser-svelte";
import { accountStore } from "$lib/stores/account.svelte";
import { TableStore } from "$lib/stores/table.svelte";
import { transactionStore } from "$lib/stores/transaction.svelte";
import { formatMoney } from "$lib/utils/money";
import TransactionTable from "./TransactionTable.svelte";

beforeEach(() => {
    transactionStore.clear();
});

describe("TransactionTable", () => {
    test("filter transactions by account id", async () => {
        const tableStore = new TableStore();
        const account = await accountStore.createTestAccount({
            name: "Test account",
        });
        transactionStore.addTestTransaction({
            fromAccountId: account.id,
            amount: "500",
        });
        transactionStore.addTestTransaction({
            id: "T2",
            fromAccountId: "does-not-exist",
            amount: "50",
        });
        const screen = await render(TransactionTable, {
            tableStore,
            accountId: account.id,
        });
        const rows = screen.getByTestId("transaction-row").all();

        expect(rows).toHaveLength(1);

        expect(screen.getByText(formatMoney("50"))).not.toBeInTheDocument();
    });
});
