import type { Category } from "./transaction.svelte";

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

export function formatDate(dateStr: string): string{
    const [year,month,day]: string[] = dateStr.split("-");
    const date = new Date(Number(year),Number(month)-1,Number(day));
    return Intl.DateTimeFormat("en-US",{ dateStyle: "medium" })
        .format(date);
}