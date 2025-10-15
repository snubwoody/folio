import type { Account, Expense } from "./lib";
import { invoke } from "@tauri-apps/api/core";

export type CreateExpense = {
	amount: string,
	date?: string,
	accountId?: string,
	categoryId?: string,
	currencyCode: string
}

export type EditExpense = {
    id: string,
	amount?: string,
	date?: string,
	accountId?: string,
	categoryId?: string,
}

export type Category = {
    id: string,
    title: string
}

export type IncomeStream = {
    id: string,
    title: string
}

export type Income = {
	id: string,
	amount: number,
	description: string,
	incomeStream?: IncomeStream,
	account?: Account,
	date: string,
	currencyCode: string
}


export class TransactionStore{
    #expenses: Expense[] = $state([]);
    #incomes: Income[] = $state([]);
    #categories: Category[] = $state([]);
    #incomeStreams: IncomeStream[] = $state([]);

    get expenses(): Expense[]{
        return this.#expenses;
    }
    
    get incomes(): Income[]{
        return this.#incomes;
    }
    
    get incomeStreams(): IncomeStream[]{
        return this.#incomeStreams;
    }
    get categories(): IncomeStream[]{
        return this.#categories;
    }
    async editExpense(opts: EditExpense){
        const { id,...data } = opts;
        await invoke("edit_expense",{ id,data });
        await this.load();
    }
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
        await this.load();
    }
        
    async load(){
        this.#expenses = await invoke("fetch_expenses") as Expense[];
        this.#categories = await invoke("fetch_categories") as Category[];
        this.#incomes = await invoke("fetch_incomes") as Income[];
        this.#incomeStreams = await invoke("fetch_income_streams") as IncomeStream[];
    }
}

export const transactionStore = new TransactionStore();
