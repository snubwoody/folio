<script lang="ts">
    import General from "./General.svelte";
    import IncomeStreams from "./IncomeStreams.svelte";
    import Categories from "./Categories.svelte";
    import MenuItem from "./MenuItem.svelte";
    let selectedIndex = $state(0);


</script>

<div class="settings-panel">
    <aside>
        <MenuItem bind:selectedIndex index={0}>General</MenuItem>
        <MenuItem bind:selectedIndex index={1}>Categories</MenuItem>
        <MenuItem bind:selectedIndex index={2}>Income streams</MenuItem>
    </aside>
    <main>
        {#if selectedIndex === 0 }
            <General/>
        {:else if selectedIndex === 1}
            <Categories/>
        {:else if selectedIndex === 2}
            <IncomeStreams/>
        {/if}
    </main>
</div>

<style>
    .settings-panel {
        display: flex;
        height: 100%;
    }

    aside{
        display: flex;
        flex-direction: column;
        gap: 8px;
        padding: 24px 16px;
        background: var(--color-neutral-25);
        height: 100%;
    }
</style>