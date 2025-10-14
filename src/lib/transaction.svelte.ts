import type { Expense } from "./lib";
import { invoke } from "@tauri-apps/api/core";


export type CreateExpense = {

} 

export class TransactionStore{
	expenses: Expense[] = $state([]);

	async addExpense(data: CreateExpense){
		try{
			await invoke("create_expense",{data})
		}
		catch(e){
			console.error(e)
		}
		await this.load()
	}

	async load(){
		// FIXME: check for error
		const expenses = await invoke("fetch_expenses") as Expense[];
		this.expenses= expenses;
	}
}

export const transactionStore = new TransactionStore()