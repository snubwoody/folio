<script lang="ts">
    import IconButton from "$components/button/IconButton.svelte";
    import Plus from "@lucide/svelte/icons/plus";
    import { appStore } from "$lib/state.svelte";
    import IncomeStream from "./IncomeStream.svelte";

    let incomeStreams = $derived.by(() => {
        return (appStore.incomeStreams ?? [])
            .toSorted(
                (a, b) =>
                    new Date(a.createdAt).getTime() -
                    new Date(b.createdAt).getTime(),
            )
            .reverse();
    });
</script>

<div class="space-y-4">
    <header class="space-y-0.5">
        <div class="flex items-center justify-between">
            <h6>Income streams</h6>
            <IconButton
                onclick={() => appStore.createIncomeStream()}
                variant="neutral"
            >
                <Plus />
            </IconButton>
        </div>
        <p class="text-sm">Income streams are used for organising incomes.</p>
    </header>
    <ul class="space-y-2">
        {#each incomeStreams as stream (stream.id)}
            <IncomeStream {stream}/>
        {/each}
    </ul>
</div>
