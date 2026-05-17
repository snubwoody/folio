import { describe, test, expect } from "vitest";
import TransactionTable from "./TransactionTable.svelte";
import { render } from "vitest-browser-svelte";
import { TableStore } from "$lib/stores/table.svelte";


describe("TransactionTable",()=>{
    test("filter transactions by account id", async () => {
        // TODO: test row header
        const tableStore = new TableStore();
        const screen = await render(TransactionTable, {tableStore});
        throw "Not implemented";
    });
});