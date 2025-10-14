<script lang="ts" generics="T">
	import {Combobox} from "melt/builders";

	// TODO: add default value
	type Props = {
		label?: string,
		items: T[],
		toOption: (item: T) => Option,
		defaultValue?: T,
		onChange?: (item: T) => void
	}

	const {label = "Label", items, toOption,onChange = (item: T)=>{},defaultValue }: Props = $props();

	type Option = {
		/** Label for display purposes. */
		label: string,
		/** Unique value for identifying. */
		value: string
	}

	const options = items.map(i => toOption(i));
	const combobox = new Combobox<Option>({
		value: defaultValue ? toOption(defaultValue):undefined,
		onValueChange: (value) => updateValue(value)
	});

	let selectedOption: Option | undefined = $state(defaultValue ? toOption(defaultValue) : undefined);
	function updateValue(value?: Option){
		selectedOption = value;
		const item = items.filter(i => toOption(i).value == value?.value)[0];
		onChange(item);
	}
</script>

<div {...combobox.input} class="space-y-1">
	<p class="text-sm text-text-muted">{label}</p>
	<button 
		{...combobox.trigger} 
		class="flex w-full items-center justify-between px-1.5 py-1 rounded-md border border-neutral-50"
	>
		<p>{selectedOption?.label ?? "Select an option"}</p>
		<i class="ph ph-caret-down"></i>
	</button>
</div>
<ul {...combobox.content} class="popup-overlay space-y-1">
	{#each options as  option (option.value)}
		{@const selected = combobox.isSelected(option)}
		<li {...combobox.getOption(option)} data-selected={selected}>
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