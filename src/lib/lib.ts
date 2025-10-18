
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