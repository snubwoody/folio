<!--
Copyright (C) 2025 Wakunguma Kalimukwa

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
-->
<script lang="ts">
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import {SvelteMap} from "svelte/reactivity";
    import {type CalendarDate, parseDate} from "@internationalized/date";
    import type {Transaction} from "$lib/api/transaction";
    import {formatDate} from "$lib/utils/date";
    import {formatMoney} from "$lib/utils/money";
    import {categoryStore} from "$lib/stores/categories.svelte";

    let transactionMap = new SvelteMap<string,Transaction[]>();
    for (const transaction of transactionStore.transactions){
        const date = transaction.date.toString();
        let arr = transactionMap.get(date);
        if (!arr) {
            transactionMap.set(date,[transaction]);
            continue;
        }

        arr.push(transaction);
        transactionMap.set(date,arr);
    }

    for (const [date,transaction] of transactionMap.entries()){
        console.log(date,transaction)
    }
</script>

<main class="overflow-y-auto w-full">
    <div class="space-y-1.5">
        {#each transactionMap.keys() as date (date)}
            <p class="font-medium text-sm text-neutral-700">{formatDate(parseDate(date))}</p>
            {#each transactionMap.get(date) ?? [] as transaction (transaction.id)}
                <div class="flex">
                    <div class="flex-1">
                        {#if transaction.categoryId}
                            {@const category = categoryStore.categoryMap.get(transaction.categoryId)}
                            <p class="font-medium">{category?.title ?? "Category"}</p>
                        {:else}
                            <!--TODO: grey out-->
                            <p class="font-medium">Category</p>
                        {/if}
                        <p class="text-sm">Account</p>
                        <p class="text-sm">{transaction.note}</p>
                    </div>
                    <p class="font-medium">{formatMoney(transaction.amount)}</p>
                </div>

            {/each}
        {/each}
    </div>
</main>

<style>
    main{
        display: flex;
        flex-direction: column;
        padding: 24px;
        gap: 24px;
    }
</style>
