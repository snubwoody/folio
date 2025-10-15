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

export const transactionStore = createTransactionStore();

export function createTransactionStore(){
    let expenses: Expense[] = $state([]);
    let incomes: Income[] = $state([]);
    let categories: Category[] = $state([]);
    let incomeStreams: IncomeStream[] = $state([]);
    return {
        get expenses(){
            return expenses;
        },
        get incomes(){
            return incomes;
        },
        get categories(){
            return categories;
        },
        get incomeStreams(){
            return incomeStreams;
        },
        async editExpense(opts: EditExpense){
            const { id,...data } = opts;
            await invoke("edit_expense",{ id,data });
            await this.load();
        },
        async addExpense({
            amount = "0",
            accountId,
            categoryId,
            currencyCode = "USD",
            date,
        }:CreateExpense,
        ){
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
        },
        async load(){
            expenses = await invoke("fetch_expenses") as Expense[];
            categories = await invoke("fetch_categories") as Category[];
            incomes = await invoke("fetch_incomes") as Income[];
            incomeStreams = await invoke("fetch_income_streams") as IncomeStream[];
        },
    };
}
