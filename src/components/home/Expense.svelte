<script lang="ts">
    import type { Expense } from "../../lib/lib";


	type Props = {
		expense: Expense
	}

	const {expense}: Props = $props();
	const formatter = new Intl.NumberFormat("en-US",{style: "currency",currency: expense.currencyCode});
	const amount = formatter.format(expense.amount);

	function formatDate(dateStr: string): string{
		const [year,month,day]: string[] = dateStr.split("-");
		const date = new Date(Number(year),Number(month)-1,Number(day));
		return Intl.DateTimeFormat("en-US",{dateStyle: "medium"})
			.format(date)
	}

	const date = formatDate(expense.date);
</script>

<p class="table-cell">{expense.category?.title ?? "Missing category!!"}</p>
<p class="table-cell">{expense.account?.name ?? "Missing account!!"}</p>
<p class="table-cell">{date}</p>
<p class="table-cell">{amount}</p>

<style>
	.expense-table{
		display: grid;
		grid-template-columns: repeat(4,1fr);
	}	

	.table-heading{
		color: var(--color-text-muted);
		padding: 12px;
	}

	.table-cell{
		padding: 12px;
		border-right: 1px solid var(--color-neutral-50);
		border-bottom: 1px solid var(--color-neutral-50);
		border-top: 1px solid var(--color-neutral-50);
	}
</style>