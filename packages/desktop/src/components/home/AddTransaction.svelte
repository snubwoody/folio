<script lang="ts">
    import AddExpense from "./AddExpense.svelte";
    import AddIncome from "./AddIncome.svelte";

    type Props = {
        activeTab: "Expenses" | "Income"
    }

    const { activeTab }: Props = $props();
</script>

{#if activeTab === "Expenses"}
     <AddExpense/>
{:else}
    <AddIncome/>
{/if}

