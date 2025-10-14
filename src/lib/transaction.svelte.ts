import type { Expense } from "./lib";


export class TransactionStore{
	expenses: Expense[] = $state([])
}