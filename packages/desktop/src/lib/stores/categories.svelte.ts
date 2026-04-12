import { invoke } from "@tauri-apps/api/core";
import { logger } from "../logger";
import { SvelteMap } from "svelte/reactivity";

export type Category = {
    id: string;
    title: string;
    createdAt: string;
    isIncomeStream: boolean
};

export class CategoryStore {
    #categories: Category[] = $state([]);

    #categoryMap: SvelteMap<string,Category> = $derived(new SvelteMap(this.categories.map(a => [a.id,a])));
    #incomeStreamMap: SvelteMap<string,Category> = $derived(new SvelteMap(this.incomeStreams.map(a => [a.id,a])));

    /**
     * Returns a list of all the categories
     */
    get categories(): Category[]{
        return this.#categories.filter(c => !c.isIncomeStream);
    }

    get allCategories(): Category[]{
        return this.#categories;
    }

    /**
     * Returns a list of all the income streams
     */
    get incomeStreams(): Category[]{
        return this.#categories.filter(c => c.isIncomeStream);
    }

    get categoryMap(){
        return this.#categoryMap;
    }

    get incomeStreamMap(){
        return this.#incomeStreamMap;
    }

    /**
     * Delete a category from the user store. Any transactions
     * referencing this category will have their category field
     * set to `null`.
     *
     * @param id The id of the {@link Category} to delete
     */
    async deleteCategory(id: string) {
        await invoke("delete_category", { id });
        this.#categories = this.#categories.filter((c) => c.id !== id);
    }

    async editCategory(id: string, title: string) {
        const category = await invoke<Category>("edit_category", { id, title });
        const index = this.#categories.findIndex((c) => c.id === category.id);
        this.#categories[index] = category;
        await this.load();
    }

    /**
     * Creates a new category.
     *
     * @param title The title of the category
     * @returns The new category
     */
    async createCategory(title: string = "New category"):Promise<Category> {
        const category = await invoke<Category>("create_category", {
            title
        });
        this.#categories.push(category);
        return category;
    }

    /**
     * Creates a new income stream.
     *
     * @param title The title of the category
     * @returns The new category
     */
    async createIncomeStream(title: string = "New income stream"):Promise<Category> {
        const category = await invoke<Category>("create_income_stream", {
            title
        });
        this.#categories.push(category);
        return category;
    }

    async load() {
        this.#categories = await invoke<Category[]>("fetch_categories");
        logger.debug("Loaded categories from backend");
    }

    clear() {
        this.#categories = [];
    }
}

export const categoryStore = new CategoryStore();

