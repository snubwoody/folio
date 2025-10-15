<!--
@component
A select menu that displays a popup with options for the user to choose
from.

# Example
```svelte
<script>
    const paymentMethods = [
        {id: 298024, title: "Apple Pay"},
        {id: 232353, title: "Klarna"},
        {id: 368547, title: "Bank"},
        {id: 869822, title: "Credit card"},
        {id: 724024, title: "Cash"},
        {id: 239142, title: "PayPal"},
    ]
</script>

<SelectMenu
    label = "Payment method"
    items={paymentMethods}
    toOption = {(item) => {value: item.id, label: item.title}}
    onChange = {(item) => console.log(item)}
/>
```
-->
<script lang="ts" generics="T">
    import { useSelect, type SelectOption } from "$lib/select.svelte";

	// TODO: add default value
	type Props = {
		label?: string,
		items: T[],
		toOption: (item: T) => SelectOption,
		defaultValue?: T,
		onChange?: (item: T) => void
	}

	const { label = "Label", items, toOption,onChange,defaultValue }: Props = $props();

	let selectedOption: SelectOption | undefined = $state(defaultValue ? toOption(defaultValue) : undefined);
	
    function updateValue(item: T,option: SelectOption){
	    selectedOption = option;
	    if(item){
	        onChange?.(item);
	    }
	}

    const {select,options} = useSelect({
        onChange: ({ item, option }) => updateValue(item, option),
        items: items,
        toOption: toOption
    });
</script>

<div class="space-y-1">
	<p class="text-sm text-text-muted">{label}</p>
	<button
		{...select.trigger}
		class="flex w-full items-center justify-between px-1.5 py-1 rounded-md border border-neutral-50"
	>
		<p>{selectedOption?.label ?? "Select an option"}</p>
		<i class="ph ph-caret-down"></i>
	</button>
</div>
<ul {...select.content} class="popup-overlay space-y-1">
	{#each options as  option (option.value)}
		{@const selected = select.isSelected(option)}
		<li {...select.getOption(option)} data-selected={selected}>
			{option.label}
		</li>
	{/each}
</ul>

<style>
	li{
		border-radius: var(--radius-sm);
		padding: 8px;
		transition: all 250ms;

		&:hover{
			background-color: var(--color-neutral-50);
		}

		&[data-selected="true"]{
			background-color: var(--color-neutral-50);
		}
	}
</style>
