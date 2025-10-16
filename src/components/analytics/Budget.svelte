<script lang="ts">
    import { formatDate, type Budget } from "$lib/lib";
    import { useSelect } from "$lib/select.svelte";
    import { appStore } from "$lib/state.svelte";

	type Props = {
		budget: Budget
	}

	const { budget }: Props = $props();
    // FIXME: use app currency code;
	const formatter = new Intl.NumberFormat("en-US",{ style: "currency",currency: "USD" });
	const amount = formatter.format(parseFloat(budget.amount));
</script>

<div class="data-cell flex justify-between items-center">
    <p>{budget.category?.title ?? " "}</p>
</div>
<p class="data-cell">{amount}</p>
