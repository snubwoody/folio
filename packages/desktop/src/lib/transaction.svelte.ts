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
import type { AppStore } from "./state.svelte";
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
    /**
     * Create a new expense.
     *
     * @param opts - The options for creating the new expense.
     */
    async addExpense(opts:CreateExpense){
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

