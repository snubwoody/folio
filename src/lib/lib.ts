import type { Category } from "./transaction.svelte"

export type Account = {
	id: string,
	name: string,
	startingBalance: number
}

export type Expense = {
	id: string,
	amount: number,
	description: string,
	category?: Category,
	account?: Account,
	date: string,
	currencyCode: string
}
