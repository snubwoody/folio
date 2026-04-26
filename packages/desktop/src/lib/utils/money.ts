import { settingsStore } from "$lib/stores/settings.svelte";
import { invoke } from "@tauri-apps/api/core";
import type {Currency} from "$lib/types";

export interface MoneyFormatOpts {
    /** Truncate large values */
    compact?: true;
    currency?: string;
}

export function formatMoney(amount: string, opts?: MoneyFormatOpts): string {
    // FIXME: use default in transactions
    const currency = opts?.currency ?? settingsStore.currency.symbol ?? settingsStore.currency.code;

    const raw = formatAmountWithoutSymbol(amount,opts);

    // The browser automatically formats RTL for certain languages
    // TODO: test this
    return `${currency}${raw}`;
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
 */
export async function getCurrencies() {
    return await invoke<Currency[]>("currencies");
}