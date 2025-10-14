export type Account = {
	id: string,
	name: string,
	startingBalance: number
}

export type Category = {
	id: string,
	title: string
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
