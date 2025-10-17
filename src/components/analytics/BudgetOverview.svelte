<script lang="ts">
    import { formatAmount } from "$lib/lib";
    import { appStore } from "$lib/state.svelte";

    const now = new Date();
    const startOfMonth = new Date(now.getFullYear(),now.getMonth(),1);
    const nextMonth = new Date(now.getFullYear(),now.getMonth() + 1,1);
    const income = appStore.incomes.filter((income) => {
        const date = new Date(income.date);
        return date >= startOfMonth && date < nextMonth
    });
    const expenses = appStore.expenses.filter((expense) => {
        const date = new Date(expense.date);
        return date >= startOfMonth && date < nextMonth
    });
    const budgets = appStore.budgets;
    const totalExpenses = $derived.by(()=>expenses.reduce((acc,item) => acc + parseFloat(item.amount),0));
    const totalIncome = $derived.by(()=>income.reduce((acc,item) => acc + parseFloat(item.amount),0));
    const totalBudget = $derived.by(()=>budgets.reduce((acc,item) => acc + parseFloat(item.amount),0));
    // TODO: format large values
    const percentage = $derived.by(()=>(totalExpenses/totalIncome) * 100)
    // FIXME: nan percentage
</script>

<section class="flex items-center justify-between">
    <div>
        <h5>Montly budget</h5>
        <h3>{formatAmount(totalBudget.toString())}</h3>
    </div>
    <div>
        <h5>Expenses</h5>
        <h3>{formatAmount(totalExpenses.toString())}</h3>
        <p class="text-sm">{percentage > 0 ? percentage.toFixed(0) : "null"}% of income</p>
    </div>
    <div>
        <h5>Income</h5>
        <h3>{formatAmount(totalIncome.toString())}</h3>
    </div>
</section>

<style>
    h5{
        margin-bottom: 20px;
    }

    h3{
        margin-bottom: 8px;
    }
</style>