import { SvelteSet } from "svelte/reactivity";

export class TableStore{
    #selectedRows: SvelteSet<string> = $state(new SvelteSet());
    #allRowsSelected: boolean = $state(false);

    get selectedRows() {
        return this.#selectedRows;
    }

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

    /// Selects a row
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
