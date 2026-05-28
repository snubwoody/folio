// TODO: detect and set date format
export type Column = "Date" | "Category" | "Note" | "Outflow" | "Inflow" | "Unknown";

export class CsvLoader{
    #rows: string[][];
    #columns: Column[];

    constructor(rows: string[][]) {
        this.#rows = $state(rows);
        let length = rows[0].length;
        let columns: Column[] = [
            "Date",
            "Category",
            "Note",
            "Outflow",
            "Inflow",
            "Unknown"
        ];

        this.#columns = $state([]);
        for (let i = 0; i < length; i++) {
            this.#columns.push(columns[i]);
        }
    }

    /**
     * Returns a list of all the csv rows.
     */
    get rows(): string[][]{
        return this.#rows;
    }


    /**
     * Returns a list of the csv columns.
     */
    get columns(): Column[]{
        return this.#columns;
    }

    load(rows: string[][]) {
        for (const row of rows) {
            this.#rows.push(row);
        }
    }

    /**
     * Deletes the row at the specified index.
     *
     * @param index Zero-based index of the row to remove.
     * - If `index < 0` or `index >= length` then no rows are removed.
     */
    deleteRow(index: number) {
        if (index < 0) return;
        if (index >= this.#rows.length) return;
        this.#rows.splice(index,1);
    }

    /**
     * Sets the column at the specified index.
     * @param index The index of the column to set.
     * @param column The column type.
     */
    setColumn(index: number, column: Column){

    }

}