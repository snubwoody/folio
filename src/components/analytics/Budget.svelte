<script lang="ts">
    import { formatAmount, formatDate, type Budget } from "$lib/lib";
    import { useSelect } from "$lib/select.svelte";
    import { appStore } from "$lib/state.svelte";

	type Props = {
		budget: Budget
	}

	const { budget }: Props = $props();
    // FIXME: use app currency code;
</script>

<div class="data-cell flex justify-between items-center">
    <p>{budget.category?.title ?? " "}</p>
</div>
<p class="data-cell">{formatAmount(budget.amount)}</p>
<p class="data-cell">{formatAmount(budget.totalSpent)}</p>
<p class="data-cell">{formatAmount(budget.remaining)}</p>
