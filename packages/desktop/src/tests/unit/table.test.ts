import { test,expect } from "vitest";
import { type DataCellParams, type DataColumn, type DataRow, DataTable } from "$lib/table";

test("Assign table columns", () => {
    const columns: DataColumn[] = [
        { id:"Account" },
        { id:"Creation date" }
    ];

    const cells: DataCellParams[] = [
        { value:"Credit Card" },
        { value:"Nov 24, 2014" },
        { value:"Savings" },
        { value:"Jan 23, 2025" }
    ];
    const table = new DataTable(columns,cells);
    expect(table.cells[0].columnId).toBe(columns[0].id);
    expect(table.cells[1].columnId).toBe(columns[1].id);
    expect(table.cells[2].columnId).toBe(columns[0].id);
    expect(table.cells[3].columnId).toBe(columns[1].id);
});