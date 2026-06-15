import { invoke } from "@tauri-apps/api/core";
import { settingsStore } from "$lib/stores/settings.svelte";
import type { Currency } from "$lib/types";

export interface MoneyFormatOpts {
    /** Truncate large values. */
    compact?: true;
    /** The ISO currency code. */
    currency?: string;
    /** Strip the currency symbol from the final value */
    stripSymbol?: boolean;
}

/**
 * Formats a monetary value.
 * @param amount
 * @param opts
 * @returns
 */
export function formatMoney(amount: string, opts?: MoneyFormatOpts): string {
    const currency = opts?.currency ?? settingsStore.currency.code;

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
        notation,
    });

    const parts = formatter.formatToParts(parseFloat(amount));

    const stripped = parts
        .filter((part) => part.type !== "currency")
        .map((part) => part.value)
        .join("")
        .trim();

    const symbol =
        opts?.currency ??
        settingsStore.currency.symbol ??
        settingsStore.currency.code;

    if (opts?.stripSymbol === true) {
        return stripped;
    }

    // The browser automatically converts to RTL for certain languages
    return `${symbol}${stripped}`;
}

/**
 * Formats a monetary value without the currency symbol.
 * @param amount
 * @param opts
 * @deprecated use formatMoney instead
 * @returns
 */
export function formatAmountWithoutSymbol(
    amount: string,
    opts?: MoneyFormatOpts,
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
        notation,
    });

    const parts = formatter.formatToParts(parseFloat(amount));
    // Filter out the currency symbol part
    return parts
        .filter((part) => part.type !== "currency")
        .map((part) => part.value)
        .join("");
}

/**
 * Parse money input from the user
 */
export function parseMoney(value: string): string | undefined {
    value = value.replaceAll(",", "");
    const amount = parseFloat(value);
    if (!Number.isNaN(amount)) return amount.toString();
    return undefined;
}

/**
 * Fetches a list of all the supported currencies.
 */
export async function getCurrencies() {
    return await invoke<Currency[]>("currencies");
}
