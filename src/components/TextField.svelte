<script lang="ts">
	type Props = {
		label: string,
		value: string,
		placeholder?: string
	}

	let {
	    label = "Label",
	    placeholder,
	    value = $bindable(),
	}: Props = $props();
</script>

<div class="flex flex-col gap-1">
	<p class="text-sm text-text-muted">{label}</p>
	<input bind:value type="text" {placeholder}>
</div>

<style>
	input{
		border: 1px solid var(--color-neutral-50);
		padding: 8px 12px;
		border-radius: var(--radius-md);
		transition: all 150ms;
		outline: none;

		&:focus{
			border-color: var(--color-purple-500);
		}
	}
</style>
