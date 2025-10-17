<script lang="ts">
    import { formatAmount, formatDate, type Income } from "$lib/lib";
    import { useSelect } from "$lib/select.svelte";
    import { appStore } from "$lib/state.svelte";
    import DatePicker from "../../DatePicker.svelte";

	type Props = {
		income: Income
	}

	const { income }: Props = $props();
	const amount = formatAmount(income.amount,{currency: income.currencyCode});

    const { select,options } = useSelect({
        items: appStore.incomeStreams, // FIXME: income streams
        toOption: (stream) => {return { value: stream.id, label: stream.title };},
        onChange: ({ item }) => appStore.transactions.editIncome({ id: income.id,incomeStreamId: item.id }),
    });

    const { select: accountSelect,options: accountSelectOpts } = useSelect({
        items: appStore.accounts,
        toOption: (account) => {return { value: account.id, label: account.name };},
        onChange: ({ item }) => appStore.transactions.editIncome({ id: income.id,accountId: item.id }),
    });
</script>

<div class="data-cell flex justify-between items-center">
    <p>{income.incomeStream?.title ?? " "}</p>
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
    <p>{income.account?.name ?? " "}</p>
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
	<p>{formatDate(income.date)}</p>
	<DatePicker/>
</li>
<p class="data-cell">{amount}</p>
