
export type NumberFormatOpts = {
    /** Truncate large values */
    compact?: true;
    currency?: string;
};
export function formatAmount(amount: string, opts?: NumberFormatOpts): string {
    // TODO: use appStore currency
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
    return formatter.format(parseFloat(amount));
}

// FIXME: join with above
export function formatAmountWithoutSymbol(
    amount: string,
    opts?: NumberFormatOpts
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

export const localCurrencySymbols: Record<string,string> = {
    USD:"$",
    CAD:"$",
    AUD:"$",
    NZD:"$",
    SGD:"$",
    ZMW:"K",
    ZAR:"R",
};

export function getCurrencySymbol(currencyCode: string = "USD"): string {
    const formatter = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: currencyCode
    });
    const parts = formatter.formatToParts(1);
    const currencyPart = parts.find((part) => part.type === "currency");
    return currencyPart?.value ?? currencyCode;
}
