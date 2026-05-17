import { SvelteSet } from "svelte/reactivity";

export class TableStore{
    #selectedRows: SvelteSet<string> = $state(new SvelteSet());
    #allRowsSelected: boolean = $state(false);

    get selectedRows() {
        return this.#selectedRows;
    }

    /**
     * Returns true if all the table rows are selected.
     */
    get allRowsSelected():  boolean{
        return this.#allRowsSelected;
    }

    isSelected(id: string): boolean{
        return this.#selectedRows.has(id) || this.#allRowsSelected;
    }

    /**
     * Toggle selects a row
     * @param id - The row of the id to select
     */
    toggleSelect(id: string){
        if (this.#selectedRows.has(id)){
            this.deselect(id);
            return;
        }
        this.select(id);
    }

    /**
     * Select a row.
     * @param id The id of the table row to select.
     */
    select(id: string){
        this.#selectedRows.add(id);
    }

    deselect(id: string){
        this.#selectedRows.delete(id);
    }

    toggleSelectAll(){
        this.#allRowsSelected = !this.#allRowsSelected;
        if(!this.#allRowsSelected){
            this.#selectedRows.clear();
        }
    }

    selectAll(){
        this.#allRowsSelected = true;
    }

    deselectAll(){
        this.#allRowsSelected = false;
        this.#selectedRows.clear();
    }

    clear() {
        this.#allRowsSelected = false;
        this.#selectedRows.clear();
    }
}
