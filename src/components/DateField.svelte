<script lang="ts">
	type Props = {
		onChange?: (year: number,month: number,day: number) => void
	}

    const { onChange }: Props = $props();
    // The date is always parsed as YYYY-MM-DD regardless of the locale.
    let selectedDate: string | undefined= $state(undefined);

    $effect(()=>{
        if (!selectedDate) return;
        let [year,month,day] = selectedDate.split("-").map(d => parseInt(d));
        onChange?.(year,month,day);
    });
</script>

<label class="flex h-full w-full flex-col gap-1">
    <p class="text-sm text-text-muted">Date</p>
    <input bind:value={selectedDate} type="date" name="" id="">
</label>

<style>
    input{
		display: flex;
		gap: 12px;
		border: 1px solid var(--color-neutral-50);
		padding: 8px 12px;
		border-radius: var(--radius-sm);
        outline: none;
        transition: all 150ms;

        &:focus{
            border-color: var(--color-purple-500);
        }
	}

    /* Hide default calendar button */
    input[type="date"]::-webkit-inner-spin-button,
    input[type="date"]::-webkit-calendar-picker-indicator {
        display: none;
        appearance: none;
    }
</style>
