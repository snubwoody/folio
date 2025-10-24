// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
import { invoke } from "@tauri-apps/api/core";
import { TransactionStore } from "./transaction.svelte";
import type { IncomeAnalytic, SpendingAnalytic,Category,IncomeStream, Account,Income,Expense, Budget } from "./lib";
import { AccountStore } from "./account.svelte";

// TODO: just manage state manually
export class AppStore{
    expenses: Expense[] = $state([]);
    incomes: Income[] = $state([]);
    categories: Category[] = $state([]);
    incomeStreams: IncomeStream[] = $state([]);
    spendingAnaltics: SpendingAnalytic[] = $state([]);
    incomeAnalytics: IncomeAnalytic[] = $state([]);
    accounts: Account[] = $state([]);
    budgets: Budget[] = $state([]);

    transactions = new TransactionStore(this);
    accountStore = new AccountStore(this);

    async createBudget(amount: string,categoryId: string){
        await invoke("create_budget",{ amount,categoryId });
        await this.load();
    }

    async load(){
        this.expenses = await invoke("fetch_expenses") as Expense[];
        this.categories = await invoke("fetch_categories") as Category[];
        this.incomes = await invoke("fetch_incomes") as Income[];
        this.incomeStreams = await invoke("fetch_income_streams") as IncomeStream[];
        this.incomeAnalytics = await invoke("income_analytics") as IncomeAnalytic[];
        // TODO: get this on demand
        this.spendingAnaltics = await invoke("spending_analytics") as SpendingAnalytic[];
        this.accounts = await invoke("fetch_accounts") as Account[];
        this.budgets = await invoke("fetch_budgets") as Budget[];
    }
}

export const appStore = new AppStore();