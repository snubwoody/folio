<script lang="ts">
    import { formatDate, type Expense } from "$lib/lib";
    import { useSelect } from "$lib/select.svelte";
    import { appStore } from "$lib/state.svelte";
    import DatePicker from "../../DatePicker.svelte";

	type Props = {
		expense: Expense
	}

	const { expense }: Props = $props();
	const formatter = new Intl.NumberFormat("en-US",{ style: "currency",currency: expense.currencyCode });
	const amount = formatter.format(expense.amount);

    const { select,options } = useSelect({
        items: appStore.categories,
        toOption: (category) => {return { value: category.id, label: category.title };},
        onChange: ({ item }) => appStore.transactions.editExpense({ id: expense.id,categoryId: item.id }),
    });

    const { select: accountSelect,options: accountSelectOpts } = useSelect({
        items: appStore.accounts,
        toOption: (account) => {return { value: account.id, label: account.name };},
        onChange: ({ item }) => appStore.transactions.editExpense({ id: expense.id,accountId: item.id }),
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
<div class="data-cell flex justify-between items-center">
    <p>{expense.account?.name ?? " "}</p>
    <button {...accountSelect.trigger} class="icon-btn icon-btn-small icon-btn-neutral"><i class="ph ph-caret-down"></i></button>
    <ul {...accountSelect.content} class="popup-overlay space-y-1 w-fit! overflow-auto">
        {#each accountSelectOpts as  option (option.value)}
            {@const selected = accountSelect.isSelected(option)}
            <li {...accountSelect.getOption(option)} data-selected={selected} class="menu-item w-fit min-w-[min-content]">
                {option.label}
            </li>
        {/each}
    </ul>
</div>
<li class="data-cell flex items-center justify-between">
	<p>{formatDate(expense.date)}</p>
	<DatePicker/>
</li>
<p class="data-cell">{amount}</p>
