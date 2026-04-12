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
import type { Category } from "./stores/categories.svelte";
import { CalendarDate } from "@internationalized/date";
import { settingsStore } from "./stores/settings.svelte";

export type Account = {
    id: string;
    name: string;
    startingBalance: string;
    balance: string;
};

export type Budget = {
    id: string;
    amount: string;
    totalSpent: string;
    remaining: string;
    category: Category;
};

export type IncomeAnalytic = {
    stream: Category;
    total: string;
};

export type Settings = {
    /**
     * The global currency code.
     */
    currencyCode: string;
};

/**
 * @deprecated
 */
export type SupportResponse = {
    issue_url: string;
    issue_id: number;
};

export function formatDate(dateStr: string): string {
    const [year, month, day]: string[] = dateStr.split("-");
    const date = new Date(Number(year), Number(month) - 1, Number(day));
    return Intl.DateTimeFormat("en-US", { dateStyle: "medium" }).format(date);
}

export interface MoneyFormatOpts {
    /** Truncate large values */
    compact?: true;
    currency?: string;
};

export function formatMoney(amount: string, opts?: MoneyFormatOpts): string {
    const currency = opts?.currency ?? settingsStore.settings.currencyCode;
    let notation:
        | "compact"
        | "standard"
        | "scientific"
        | "engineering"
        | undefined;
    if (opts?.compact) {
        notation = "compact";
    }

    const formatter = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency,
        notation
    });
    return formatter.format(parseFloat(amount));
}

// FIXME: join with above
export function formatAmountWithoutSymbol(
    amount: string,
    opts?: MoneyFormatOpts
): string {
    const currency = opts?.currency ?? "USD";
    let notation:
        | "compact"
        | "standard"
        | "scientific"
        | "engineering"
        | undefined;
    if (opts?.compact) {
        notation = "compact";
    }

    const formatter = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency,
        notation
    });

    const parts = formatter.formatToParts(parseFloat(amount));
    // Filter out the currency symbol part
    return parts
        .filter((part) => part.type !== "currency")
        .map((part) => part.value)
        .join("");
}

export function getCurrencySymbol(currencyCode: string = "USD"): string {
    const formatter = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: currencyCode
    });
    const parts = formatter.formatToParts(1);
    const currencyPart = parts.find((part) => part.type === "currency");
    return currencyPart?.value ?? currencyCode;
}

/**
 * Parse money input from the user
 */
export function parseMoney(value: string): string | undefined {
    value = value.replaceAll(",", "");
    const amount = parseFloat(value);
    if (!isNaN(amount)) return amount.toString();
    return undefined;
}

/**
 * Fetches a list of all the supported currencies.
 *
 * @returns A list of currency strings
 */
export async function getCurrencies(){
    return await invoke("currencies") as string[];
}

// The goal of this function to parse the input and always return a valid date.
export function parseDate(value: string,reference: CalendarDate): CalendarDate{
    const singleDigit = /^\d+$/.test(value.trim());
    if (singleDigit){
        return reference.set({ day: parseInt(value.trim()) });
    }
    // Test constraints
    console.log(singleDigit);

    // return previous date if all else failes
    return reference;

}