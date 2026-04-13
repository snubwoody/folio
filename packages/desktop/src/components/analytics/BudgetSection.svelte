<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2025 Wakunguma Kalimukwa
-->
<script lang="ts">
    import {appStore, getBudget} from "$lib/state.svelte";
    import {categoryStore} from "$lib/stores/categories.svelte";
    import Budget from "./Budget.svelte";
    // TODO: key by transactions
    // FIXME: remove cell background when focused
</script>

<section>
    <ul class="budget-table">
        <div class="flex items-center gap-1.5">
            <h6>Categories</h6>
        </div>
        <h6>Total</h6>
        <h6>Left to spend</h6>
        <!-- <p class="table-heading">Remaining</p> -->
        {#each categoryStore.categories as category (category.id)}
            {#await getBudget(category.id)}
                Loading
            {:then budget}
                <Budget budget={budget}/>
            {/await}
            <!--{#key budget.amount}-->
<!--                <Budget {budget}/>-->
<!--            {/key}-->
            <div class="bg-neutral-50 w-full h-[1px] col-span-3"></div>
        {/each}
    </ul>
</section>

<style>
	.budget-table{
		display: grid;
		grid-template-columns: 1fr auto auto;
        align-items: center;
        gap: 16px 56px;
	}
</style>
