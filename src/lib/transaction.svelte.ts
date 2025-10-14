import type { Expense } from "./lib";
import { invoke } from "@tauri-apps/api/core";

export type CreateExpense = {
	amount: string,
	date?: string,
	accountId?: string,
	categoryId?: string,
	currencyCode: string
}

export type Category = {
    id: string,
    title: string
}

export const transactionStore = createTransactionStore();

export function createTransactionStore(){
    let expenses: Expense[] = $state([]);
    let categories: Category[] = $state([]);
    return {
        get expenses(){
            return expenses;
        },
        get categories(){
            return categories;
        },
        async addExpense({
            amount = "0",
            accountId,
            categoryId,
            currencyCode = "USD",
            date,
        }:CreateExpense,
        ){
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
        },
        async load(){
            expenses = await invoke("fetch_expenses") as Expense[];
            categories = await invoke("fetch_categories") as Category[];
        },
    };
}
