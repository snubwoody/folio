import { test,expect } from "vitest";
import { type DataCellParams, type DataColumn, type DataRow, DataTable } from "$lib/table";

test("Assign table columns", () => {
    const columns: DataColumn[] = [
        { id:"Account" },
        { id:"Creation date" }
    ];

    const rows: DataRow[] = [
        { id: "1" },
        { id: "2" },
        { id: "3" },
        { id: "4" },
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
        { id: "4" },
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