import { test,expect } from "vitest";
import { type DataCellParams, type DataColumn, type DataRow, DataTable } from "$lib/table";
import { describe } from "node:test";
import { TableStore } from "$lib/stores/table.svelte";

describe("TableStore",()=>{
    test(".toggleSelectAll selects all rows",()=>{
        const store = new TableStore();
        store.toggleSelectAll();

        expect(store.isSelected("")).toBe(true);
    });
    test(".select selects a row",()=>{
        const store = new TableStore();
        store.select("id1");

        expect(store.isSelected("id1")).toBe(true);
        expect(store.isSelected("id2")).toBe(false);
    });
    test(".deselect deselects a row",()=>{
        const store = new TableStore();
        store.select("id1");
        expect(store.isSelected("id1")).toBe(true);
        store.deselect("id1");
        expect(store.isSelected("id1")).toBe(false);
    });
    test(".toggleSelect selects and deselects a row",()=>{
        const store = new TableStore();
        store.toggleSelect("id1");
        expect(store.isSelected("id1")).toBe(true);
        store.toggleSelect("id1");
        expect(store.isSelected("id1")).toBe(false);
    });
});

test("Assign table columns", () => {
    const columns: DataColumn[] = [
        { id:"Account" },
        { id:"Creation date" }
    ];

    const rows: DataRow[] = [
        { id: "1" },
        { id: "2" },
        { id: "3" },
        { id: "4" }
    ];

    const cells: DataCellParams[] = [
        { value:"Credit Card" },
        { value:"Nov 24, 2014" },
        { value:"Savings" },
        { value:"Jan 23, 2025" }
    ];

    const table = new DataTable(columns,rows,cells);
    expect(table.cells[0].columnId).toBe(columns[0].id);
    expect(table.cells[1].columnId).toBe(columns[1].id);
    expect(table.cells[2].columnId).toBe(columns[0].id);
    expect(table.cells[3].columnId).toBe(columns[1].id);
});

test("Assign table rows", () => {
    const columns: DataColumn[] = [
        { id:"Account" },
        { id:"Creation date" }
    ];

    const rows: DataRow[] = [
        { id: "1" },
        { id: "2" },
        { id: "3" },
        { id: "4" }
    ];

    const cells: DataCellParams[] = [
        { value:"Credit Card" },
        { value:"Nov 24, 2014" },
        { value:"Savings" },
        { value:"Jan 23, 2025" }
    ];

    const table = new DataTable(columns,rows,cells);
    expect(table.cells[0].rowId).toBe(rows[0].id);
    expect(table.cells[1].rowId).toBe(rows[0].id);
    expect(table.cells[2].rowId).toBe(rows[1].id);
    expect(table.cells[3].rowId).toBe(rows[1].id);
});