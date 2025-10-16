import { invoke } from "@tauri-apps/api/core";
import type { AppStore } from "./state.svelte";

export type CreateExpense = {
	amount: string,
	date?: string,
	accountId?: string,
	categoryId?: string,
	currencyCode: string
}

export type CreateIncome = {
	amount: string,
	date?: string,
	accountId?: string,
	incomeStreamId?: string,
	currencyCode: string
}

export type EditExpense = {
    id: string,
	amount?: string,
	date?: string,
	accountId?: string,
	categoryId?: string,
}

/** Data used for editing an `Income`,
 * values left as `undefined` or `null`  will be
 * left as their current values.
 *
 */
export type EditIncome = {
    id: string,
	amount?: string,
	date?: string,
	accountId?: string,
	incomeStreamId?: string,
}

export class TransactionStore{
    #rootStore: AppStore;
    constructor(root: AppStore){
        this.#rootStore = root;
    }

    async editExpense(opts: EditExpense){
        const { id,...data } = opts;
        await invoke("edit_expense",{ id,data });
        await this.#rootStore.load();
    }
    async editIncome(opts: EditIncome){
        const { id,...data } = opts;
        await invoke("edit_income",{ id,data });
        await this.#rootStore.load();
    }
    /**
     * Create a new expense.
     *
     * @param opts - The options for creating the new expense.
     */
    async addExpense(opts:CreateExpense){
        const {
            amount = "0",
            accountId,
            categoryId,
            currencyCode = "USD",
            date,
        } = opts;
        // TODO: use default currency code
        const data: CreateExpense = {
            amount,
            accountId,
            currencyCode,
            categoryId,
            date,
        };

        try{
            await invoke("create_expense",{ data });
        }
        catch(e){
            console.error(e);
        }
        await this.#rootStore.load();
    }
    /**
     * Create a new expense.
     *
     * @param opts - The options for creating the new expense.
     */
    async addIncome(opts:CreateIncome){
        // TODO: make amount nullable and currency code nullable on backend
        const {
            amount = "0",
            accountId,
            incomeStreamId,
            currencyCode = "USD",
            date,
        } = opts;
        // TODO: use default currency code
        const data: CreateIncome = {
            amount,
            accountId,
            currencyCode,
            incomeStreamId,
            date,
        };

        try{
            await invoke("create_income",{ data });
        }
        catch(e){
            console.error(e);
        }
        await this.#rootStore.load();
    }
}

