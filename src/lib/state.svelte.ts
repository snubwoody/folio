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
import type {
    IncomeAnalytic,
    SpendingAnalytic,
    Category,
    IncomeStream,
    Account,
    Income,
    Expense,
    Budget,
    Settings
} from "./lib";
import { AccountStore } from "./account.svelte";

// TODO: just manage state manually
export class AppStore {
    expenses: Expense[] = $state([]);
    incomes: Income[] = $state([]);
    categories: Category[] = $state([]);
    incomeStreams: IncomeStream[] = $state([]);
    spendingAnaltics: SpendingAnalytic[] = $state([]);
    incomeAnalytics: IncomeAnalytic[] = $state([]);
    accounts: Account[] = $state([]);
    budgets: Budget[] = $state([]);

    settings: Settings = $state({currencyCode: "USD"});
    transactions = new TransactionStore(this);
    accountStore = new AccountStore(this);

    async createBudget(amount: string, categoryId: string) {
        await invoke("create_budget", { amount, categoryId });
        await this.load();
    }

    async setCurrencyCode(currency: string) {
        await invoke("set_currency_code", { currency });
        this.expenses = (await invoke("fetch_expenses")) as Expense[];
        this.incomes = (await invoke("fetch_incomes")) as Income[];
        this.settings = (await invoke("settings")) as Settings;
    }

    async editBudget(id: string, amount: string) {
        await invoke("edit_budget", { id, amount });
        await this.load();
    }

    async deleteBudget(id: string) {
        await invoke("delete_budget", { id });
        await this.load();
    }

    /**
     * Delete a category from the user store. Any transactions
     * referrencing this category will have their category field
     * set to `null`.
     *
     * @param id The id of the {@link Category} to delete
     */
    async deleteCategory(id: string) {
        await invoke("delete_category", { id });
        this.categories = this.categories.filter((c) => c.id !== id);
        this.expenses = (await invoke("fetch_expenses")) as Expense[];
        this.spendingAnaltics = (await invoke(
            "spending_analytics",
        )) as SpendingAnalytic[];
    }

    async editCategory(id: string, title: string) {
        await invoke("edit_category", { id, title });
        this.categories = (await invoke("fetch_categories")) as Category[];
        this.expenses = (await invoke("fetch_expenses")) as Expense[];
        this.incomeAnalytics = (await invoke(
            "income_analytics",
        )) as IncomeAnalytic[];
    }

    /**
     * Delete an income stream from the user store. Any transactions
     * referrencing this income stream will have their referencing field
     * set to `null`.
     *
     * @param id The id of the {@link IncomeStream} to delete
     */
    async deleteIncomeStream(id: string) {
        await invoke("delete_income_stream", { id });
        this.incomeStreams = this.incomeStreams.filter((c) => c.id !== id);
        this.incomes = (await invoke("fetch_incomes")) as Expense[];
        this.incomeAnalytics = (await invoke(
            "income_analytics",
        )) as IncomeAnalytic[];
    }

    /**
     * Create a new {@link Category}.
     *
     * @param title The title of the category
     */
    async createCategory(title: string = "New category") {
        const category = (await invoke("create_category", {
            title,
        })) as Category;
        this.categories.push(category);
        this.spendingAnaltics = (await invoke(
            "spending_analytics",
        )) as SpendingAnalytic[];
    }

    /**
     * Create a new {@link IncomeStream}.
     *
     * @param title The title of the income stream
     */
    async createIncomeStream(title: string = "New income stream") {
        const stream = (await invoke("create_income_stream", {
            title,
        })) as IncomeStream;
        this.incomeStreams.push(stream);
        this.spendingAnaltics = (await invoke(
            "spending_analytics",
        )) as SpendingAnalytic[];
    }

    async load() {
        this.expenses = (await invoke("fetch_expenses")) as Expense[];
        this.categories = (await invoke("fetch_categories")) as Category[];
        this.incomes = (await invoke("fetch_incomes")) as Income[];
        this.incomeStreams = (await invoke(
            "fetch_income_streams",
        )) as IncomeStream[];
        this.incomeAnalytics = (await invoke(
            "income_analytics",
        )) as IncomeAnalytic[];
        // TODO: get this on demand
        this.spendingAnaltics = (await invoke(
            "spending_analytics",
        )) as SpendingAnalytic[];
        this.accounts = (await invoke("fetch_accounts")) as Account[];
        this.budgets = (await invoke("fetch_budgets")) as Budget[];
        this.settings = (await invoke("settings")) as Settings;
    }
}

export const appStore = new AppStore();
