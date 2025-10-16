import { invoke } from "@tauri-apps/api/core";
import type { Category, IncomeStream } from "./transaction.svelte";

export type SpendingAnalytic = {
    category: Category,
    total: string
}

export type IncomeAnalytic = {
    stream: IncomeStream,
    total: string
}

export class AnalyticsStore{
    #spendingAnaltics: SpendingAnalytic[] = $state([]);
    #incomeAnalytics: IncomeAnalytic[] = $state([]);

    get spendingAnalytics(): SpendingAnalytic[]{
        return this.#spendingAnaltics;
    }
    
    get incomeAnalytics(): IncomeAnalytic[]{
        return this.#incomeAnalytics;
    }

    async load(){
        this.#incomeAnalytics = await invoke("income_analytics") as IncomeAnalytic[];
    }
}

export const analyticsStore = new AnalyticsStore();
