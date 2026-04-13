// Global types

export type Category = {
    id: string;
    title: string;
    createdAt: string;
    isIncomeStream: boolean
};

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