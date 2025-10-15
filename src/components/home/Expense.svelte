<script lang="ts">
    import { formatDate, type Expense } from "$lib/lib";
    import { useSelect } from "$lib/select.svelte";
    import { transactionStore } from "$lib/transaction.svelte";
    import DatePicker from "../DatePicker.svelte";

	type Props = {
		expense: Expense
	}

	const { expense }: Props = $props();
	const formatter = new Intl.NumberFormat("en-US",{ style: "currency",currency: expense.currencyCode });
	const amount = formatter.format(expense.amount);

    const {select,options} = useSelect({
        items: transactionStore.categories,
        toOption: (category) => {return {value: category.id, label: category.title}},
        onChange: ({ item }) => transactionStore.editExpense({categoryId: item.id}),
    });
</script>

<div class="data-cell flex justify-between items-center">
    <p>{expense.category?.title ?? " "}</p>
    <button {...select.trigger} class="icon-btn icon-btn-small icon-btn-neutral"><i class="ph ph-caret-down"></i></button>
    <ul {...select.content} class="popup-overlay space-y-1 w-fit! overflow-auto">
        {#each options as  option (option.value)}
            {@const selected = select.isSelected(option)}
            <li {...select.getOption(option)} data-selected={selected} class="menu-item w-fit min-w-[min-content]">
                {option.label}
            </li>
        {/each}
    </ul>
</div>
<p class="data-cell">{expense.account?.name ?? " "}</p>
<li class="data-cell flex items-center justify-between">
	<p>{formatDate(expense.date)}</p>
	<DatePicker/>
</li>
<p class="data-cell">{amount}</p>

<style>
	.data-cell{
		padding: 12px;
		border-right: 1px solid var(--color-neutral-50);
		border-bottom: 1px solid var(--color-neutral-50);
		border-top: 1px solid var(--color-neutral-50);
	}
</style>
