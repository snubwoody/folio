<script lang="ts">
    import IconButton from "$components/button/IconButton.svelte";
    import Plus from "@lucide/svelte/icons/plus";
    import Delete from "@lucide/svelte/icons/trash-2";
    import { appStore } from "$lib/state.svelte";

    // TODO: order by created_by
</script>

<div class="space-y-4">
    <header class="space-y-0.5">
        <div class="flex items-center justify-between">
            <h6>Income streams</h6>
            <IconButton variant="neutral">
                <Plus/>
            </IconButton>
        </div>
        <p class="text-sm">Income streams are used for organising incomes.</p>
    </header>
    <ul class="space-y-2">
        {#each appStore.incomeStreams as stream (stream.id)}
            <li class="flex items-center justify-between">
                <p>{stream.title}</p>
                <IconButton variant="ghost">
                    <Delete/>
                </IconButton>
            </li>
        {/each}
    </ul>
</div>