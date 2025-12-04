
export interface DataRow{ id: string }
export interface DataColumn{ id: string }

export type DataValue = string | number;

export interface DataCell extends DataCellParams{
    columnId: string
}

export interface DataCellParams {
    value: DataValue
}

export class DataTable{
    private readonly _cells: DataCell[];

    constructor(columns: DataColumn[],cells: DataCellParams[]) {
        let columnIndex = 0;
        const dataCells = [];
        for (const cell of cells) {
            const dataCell: DataCell = { columnId: columns[columnIndex].id,...cell };
            dataCells.push(dataCell);
            if (columnIndex === columns.length - 1) {
                columnIndex = 0;
            } else {
                columnIndex++;
            }
        }
        this._cells = dataCells;
    }

    get cells(): DataCell[]{
        return this._cells;
    }
}