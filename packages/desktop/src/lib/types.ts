// Global types

export type Currency = {
    code: string;
    name: string;
    precision?: number;
    symbol?: string;
};

export type Category = {
    id: string;
    title: string;
    createdAt: string;
    isIncomeStream: boolean;
    categoryGroupId?: string;
};

export type CategoryGroup = {
    id: string;
    title: string;
    sortOrder: number;
    createdAt: number;
};

export type SelectOption = {
    /** Label for display purposes. */
    label: string;
    /** Unique value for identifying. */
    value: string;
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
    categoryId: string;
};

export type Settings = {
    /**
     * The global currency code.
     */
    currencyCode: string;
    sidebarOpen: boolean;
};

export interface Analytic {
    category: Category;
    total: string;
}
