<script lang="ts">
    import BudgetTable from "./BudgetTable.svelte";
    import CreateBudget from "./CreateBudget.svelte";
</script>

<section>
    <header class="flex items-center justify-between">
        <h6>Budget</h6>
        <CreateBudget/>
    </header>
    <BudgetTable/>
</section>