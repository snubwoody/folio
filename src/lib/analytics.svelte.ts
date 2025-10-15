import { invoke } from "@tauri-apps/api/core";
import type { Category } from "./transaction.svelte";

export type SpendingAnalytic = {
    category: Category,
    total: string
}

export class AnalyticsStore{
    #spendingAnaltics: SpendingAnalytic[] = $state([]);

    get spendingAnalytics(): SpendingAnalytic[]{
        return this.#spendingAnaltics;
    }
    
    async load(){
        this.#spendingAnaltics = await invoke("spending_analytics") as SpendingAnalytic[];
    }
}

export const analyticsStore = new AnalyticsStore();