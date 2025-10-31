import type { Expense,Category } from "./lib";

export type SpendingAnalytic = {
    category: Category,
    total: number
};

/**
 * Calculates the amount spent per category.
 * @param expenses An array of expenses. 
 */
export const calculateSpendingAnalytics = (expenses: Expense[] ): SpendingAnalytic[] => {
    expenses = expenses.filter(expense => expense.category !== null);
    const categories = new Set<string>();
    expenses.forEach(expense => {
        categories.add(expense.category.id);
    });
    const analytics: SpendingAnalytic[] = [];
    for (const id of categories){
        const e = expenses
            .filter(expense => expense.category.id === id);
        const category = e[0].category;

        const total = expenses
            .filter(expense => expense.category.id === id)
            .reduce((previous,current) => previous + parseFloat(current.amount),0);

        analytics.push({category,total});
    }
    return analytics;
};