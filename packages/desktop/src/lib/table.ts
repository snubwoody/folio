
export interface DataRow{ id: string }
export interface DataColumn{ id: string }

export interface DataCell extends DataCellParams{
    columnId: string,
    rowId: string,
}

export interface DataCellParams {
    value: string,
}

export class DataTable{
    private readonly _columns: DataColumn[];
    private readonly _rows: DataRow[];
    private readonly _cells: DataCell[];

    constructor(columns: DataColumn[],rows: DataRow[],cells: DataCellParams[]) {
        let rowIndex = 0;
        let columnIndex = 0;
        const dataCells = [];
        for (const cell of cells) {
            const dataCell: DataCell = {
                columnId: columns[columnIndex].id,
                rowId: rows[rowIndex].id,
                ...cell
            };
            dataCells.push(dataCell);
            if (columnIndex === columns.length - 1) {
                columnIndex = 0;
                rowIndex++;
            } else {
                columnIndex++;
            }
        }
        this._rows = rows;
        this._columns = columns;
        this._cells = dataCells;
    }

    get cells(): DataCell[]{
        return this._cells;
    }

    get rows(): DataRow[]{
        return this._rows;
    }

    rowCells(id: string): DataCell[]{
        return this.cells.filter(cell => cell.rowId === id);
    }

    get columns(): DataColumn[]{
        return this._columns;
    }
}