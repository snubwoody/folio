import type Expense from "$components/home/expense/Expense.svelte";
import type Income from "$components/home/income/Income.svelte";
import { invoke } from "@tauri-apps/api/core";
import { TransactionStore } from "./transaction.svelte";
import type { IncomeAnalytic, SpendingAnalytic,Category,IncomeStream, Account } from "./lib";
import { AccountStore } from "./account.svelte";

export class AppStore{
    expenses: Expense[] = $state([]);
    incomes: Income[] = $state([]);
    categories: Category[] = $state([]);
    incomeStreams: IncomeStream[] = $state([]);
    spendingAnaltics: SpendingAnalytic[] = $state([]);
    incomeAnalytics: IncomeAnalytic[] = $state([]);
    accounts: Account[] = $state([]);

    transactions = new TransactionStore(this);
    accountStore = new AccountStore(this);

    async load(){
        this.expenses = await invoke("fetch_expenses") as Expense[];
        this.categories = await invoke("fetch_categories") as Category[];
        this.incomes = await invoke("fetch_incomes") as Income[];
        this.incomeStreams = await invoke("fetch_income_streams") as IncomeStream[];
        this.incomeAnalytics = await invoke("income_analytics") as IncomeAnalytic[];
        this.spendingAnaltics = await invoke("spending_analytics") as SpendingAnalytic[];
        this.accounts = await invoke("fetch_accounts") as Account[];
    }
}

export const appStore = new AppStore();