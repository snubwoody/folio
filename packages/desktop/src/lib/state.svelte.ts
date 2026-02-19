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
import type {
    IncomeAnalytic,
    Category,
    IncomeStream,
    Account,
    Income,
    Expense,
    Budget,
    Settings
} from "./lib";
import { logger } from "./logger";
import { AppError, type ErrorResponse } from "./error";


export type CreateExpense = {
    amount: string,
    date?: string,
    accountId?: string,
    categoryId?: string,
    currencyCode: string
};

export type CreateIncome = {
    amount: string,
    date?: string,
    accountId?: string,
    incomeStreamId?: string,
    currencyCode: string
};

export type EditExpense = {
    id: string,
    amount?: string,
    date?: string,
    accountId?: string,
    categoryId?: string,
};

/** Data used for editing an `Income`,
 * values left as `undefined` or `null`  will be
 * left as their current values.
 *
 */
export type EditIncome = {
    id: string,
    amount?: string,
    date?: string,
    accountId?: string,
    incomeStreamId?: string,
};

export class TransactionStore{
    #rootStore: AppStore;
    constructor(root: AppStore){
        this.#rootStore = root;
    }

    // TODO: return from data base and update local
    async editExpense(opts: EditExpense){
        const { id,...data } = opts;
        data.amount = data.amount?.replaceAll(",","");

        try{
            await invoke("edit_expense",{ id,data });

        } catch(e){
            const error = e as ErrorResponse;
            throw new AppError(error.message);
        }
        await this.#rootStore.load();
    }

    async deleteExpense(id: string){
        try{
            await invoke("delete_expense",{ id });

        } catch(e){
            const error = e as ErrorResponse;
            throw new AppError(error.message);
        }
        await this.#rootStore.load();
    }

    async deleteIncome(id: string){
        try{
            await invoke("delete_income",{ id });

        } catch(e){
            const error = e as ErrorResponse;
            throw new AppError(error.message);
        }
        await this.#rootStore.load();
    }

    async editIncome(opts: EditIncome){
        const { id,...data } = opts;
        data.amount = data.amount?.replaceAll(",","");
        try{
            await invoke("edit_income",{ id,data });

        } catch(e){
            const error = e as ErrorResponse;
            throw new AppError(error.message);
        }
        await this.#rootStore.load();
    }
    // TODO: return the expense
    /**
     * Create a new expense.
     *
     * @param opts - The options for creating the new expense.
     */
    async addExpense(opts:CreateExpense){
        // TODO: currency code should be optional
        const {
            amount = "0",
            accountId,
            categoryId,
            currencyCode = "USD",
            date
        } = opts;
        // TODO: use default currency code
        const data: CreateExpense = {
            amount,
            accountId,
            currencyCode,
            categoryId,
            date
        };

        try{
            await invoke("create_expense",{ data });
        } catch(e){
            console.error(e);
        }
        await this.#rootStore.load();
    }
    /**
     * Create a new expense.
     *
     * @param opts - The options for creating the new expense.
     */
    async addIncome(opts:CreateIncome){
        // TODO: make amount nullable and currency code nullable on backend
        const {
            amount = "0",
            accountId,
            incomeStreamId,
            currencyCode = "USD",
            date
        } = opts;
        // TODO: use default currency code
        const data: CreateIncome = {
            amount,
            accountId,
            currencyCode,
            incomeStreamId,
            date
        };

        try{
            await invoke("create_income",{ data });
        } catch(e){
            console.error(e);
        }
        await this.#rootStore.load();
    }
}

interface EditAccount{
    name?: string
    startingBalance?: string
}

export class AccountStore {
    #rootStore: AppStore;

    constructor(store: AppStore) {
        this.#rootStore = store;
    }

    async addAccount(name: string, startingBalance: string) {
        try {
            await invoke("create_account", { name, startingBalance });
        } catch (e) {
            console.error(e);
        }
        await this.#rootStore.load();
    }

    async editAccount(id: string, opts: EditAccount) {
        try {
            await invoke("edit_account", { id, opts });
        } catch (e) {
            console.error(e);
        }
        await this.#rootStore.load();
    }
}

// TODO: just manage state manually
export class AppStore {
    // TODO: add getters
    expenses: Expense[] = $state([]);
    incomes: Income[] = $state([]);
    categories: Category[] = $state([]);
    incomeStreams: IncomeStream[] = $state([]);
    accounts: Account[] = $state([]);

    budgets: Budget[] = $state([]);
    incomeAnalytics: IncomeAnalytic[] = $state([]);

    settings: Settings = $state({ currencyCode: "USD" });
    transactions = new TransactionStore(this);
    accountStore = new AccountStore(this);

    /// Returns a list of all the user's expenses.
    // get expenses(): Expense[]{
    //     return this.expenses;
    // }

    async createBudget(amount: string, categoryId: string) {
        await invoke("create_budget", { amount, categoryId });
        await this.load();
    }

    /**
     * Returns an array of sorted income analytics. The analytics
     * are sorted in descending order.
     */
    sortedIncomeAnalytics(): IncomeAnalytic[] {
        return this.incomeAnalytics
            .toSorted((a,b) => parseFloat(a.total) - parseFloat(b.total))
            .reverse();
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
        try{
            await invoke("delete_budget", { id });
        } catch(e){
            logger.error(`${e}`);
        }
        // FIXME: no longer exists
        await this.load();
    }

    /**
     * Delete a category from the user store. Any transactions
     * referencing this category will have their category field
     * set to `null`.
     *
     * @param id The id of the {@link Category} to delete
     */
    async deleteCategory(id: string) {
        await invoke("delete_category", { id });
        this.categories = this.categories.filter((c) => c.id !== id);
        this.expenses = (await invoke("fetch_expenses")) as Expense[];
    }

    async editCategory(id: string, title: string) {
        await invoke("edit_category", { id, title });
        await this.load();
    }

    /**
     * Edit an income stream.
     *
     * @param id The id of the {@link IncomeStream} to edit
     * @param title The new title
     */
    async editIncomeStream(id: string, title: string) {
        await invoke("edit_income_stream", { id, title });
        await this.load();
    }

    /**
     * Deletes an income stream from the user store. Any transactions
     * referencing this income stream will have their referencing field
     * set to `null`.
     *
     * @param id The id of the {@link IncomeStream} to delete
     */
    async deleteIncomeStream(id: string) {
        await invoke("delete_income_stream", { id });
        await this.load();
    }

    /**
     * Creates a new {@link Category}.
     *
     * @param title The title of the category
     */
    async createCategory(title: string = "New category") {
        const category = (await invoke("create_category", {
            title
        })) as Category;
        this.categories.push(category);
    }

    async deleteAccount(id: string) {
        await invoke("delete_account",{ id });
        await this.load();
    }

    /**
     * Creates a new {@link IncomeStream}.
     *
     * @param title The title of the income stream
     */
    async createIncomeStream(title: string = "New income stream") {
        const stream = (await invoke("create_income_stream", {
            title
        })) as IncomeStream;
        this.incomeStreams.push(stream);
    }

    async loadExpenses(){
        const e = (await invoke("fetch_expenses")) as Expense[];
        this.expenses = this.expenses.filter(() => false);
        for (const expense of e){
            this.expenses.push(expense);
        }
        logger.debug("Loaded expenses from database");
    }

    async load() {
        this.categories = (await invoke("fetch_categories")) as Category[];
        this.incomes = (await invoke("fetch_incomes")) as Income[];
        this.incomeStreams = (await invoke(
            "fetch_income_streams"
        )) as IncomeStream[];
        this.incomeAnalytics = (await invoke(
            "income_analytics"
        )) as IncomeAnalytic[];
        this.accounts = (await invoke("fetch_accounts")) as Account[];
        this.budgets = (await invoke("fetch_budgets")) as Budget[];
        this.settings = (await invoke("settings")) as Settings;
        await this.loadExpenses();
        logger.info("Loaded data from backend");
    }
}

export const appStore = new AppStore();
