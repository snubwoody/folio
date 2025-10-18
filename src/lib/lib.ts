
export type Account = {
	id: string,
	name: string,
	startingBalance: string
}

export type Category = {
    id: string,
    title: string
}

export type IncomeStream = {
    id: string,
    title: string
}

export type Income = {
	id: string,
	amount: string,
	description: string,
	incomeStream?: IncomeStream,
	account?: Account,
	date: string,
	currencyCode: string
}

export type Expense = {
	id: string,
	amount: string,
	description: string,
	category?: Category,
	account?: Account,
	date: string,
	currencyCode: string
}

export type Budget = {
    id: string,
    amount: string,
    totalSpent: string,
    remaining: string,
    category: Category,
}

export type SpendingAnalytic = {
    category: Category,
    total: string
}

export type IncomeAnalytic = {
    stream: IncomeStream,
    total: string
}

export function formatDate(dateStr: string): string{
    const [year,month,day]: string[] = dateStr.split("-");
    const date = new Date(Number(year),Number(month)-1,Number(day));
    return Intl.DateTimeFormat("en-US",{ dateStyle: "medium" })
        .format(date);
}

export type NumberFormatOpts = {
    /** Truncate large values */
    compact?: true,
    currency?: string
}
export function formatAmount(amount: string,opts?: NumberFormatOpts): string{
    const currency = opts?.currency ?? "USD";
    let notation: "compact" | "standard"|"scientific" | "engineering" | undefined;
    if (opts?.compact){
        notation = "compact";
    }

    const formatter = new Intl.NumberFormat("en-US",{ style: "currency",currency,notation });
    return formatter.format(parseFloat(amount));
}

// FIXME: join with above
export function formatAmountWithoutSymbol(amount: string, opts?: NumberFormatOpts): string {
    const currency = opts?.currency ?? "USD";
    let notation: "compact" | "standard"|"scientific" | "engineering" | undefined;
    if (opts?.compact){
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
        .filter(part => part.type !== "currency")
        .map(part => part.value)
        .join("");
}

export function getCurrencySymbol(currencyCode: string = "USD"): string {
    const formatter = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: currencyCode,
    });
    const parts = formatter.formatToParts(1);
    const currencyPart = parts.find(part => part.type === "currency");
    return currencyPart?.value ?? currencyCode;
}

/**
 * Parse money input from the user
 */
export function parseMoney(value: string): string | undefined{
    value = value.replaceAll(",","");
    const amount = parseFloat(value);
    if (!isNaN(amount)) return amount.toString();
    return undefined;
}