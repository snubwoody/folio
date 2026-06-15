<script lang="ts">
    import { Checkbox } from "$components/select";
    import { Table, TableCell, TableHeader } from "$components/table";
    import type { TableStore } from "$lib/stores/table.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import Transaction from "./Transaction.svelte";

    interface Props {
        tableStore: TableStore;
        accountId?: string;
    }

    const { tableStore, accountId }: Props = $props();
    let selected = $derived(tableStore.allRowsSelected);

    const select = (checked: boolean) => {
        if (checked) {
            tableStore.selectAll();
            return;
        }
        tableStore.deselectAll();
    };

    let transactions = $derived.by(() => {
        let transactions = transactionStore.transactions;

        if (accountId) {
            transactions = transactions.filter(
                (t) => t.fromAccountId === accountId,
            );
        }

        return transactions;
    });
</script>

<Table length={accountId ? 7 : 8}>
    <TableHeader>
        <Checkbox bind:checked={selected} onChecked={select}/>
        <TableCell class="data-cell-padding">Date</TableCell>
        {#if !accountId}
            <TableCell class="data-cell-padding">Account</TableCell>
        {/if}
        <TableCell class="data-cell-padding">Payee</TableCell>
        <TableCell class="data-cell-padding">Note</TableCell>
        <TableCell class="data-cell-padding">Category</TableCell>
        <TableCell class="data-cell-padding">Outflow</TableCell>
        <TableCell class="data-cell-padding">Inflow</TableCell>
    </TableHeader>
    {#each transactions as transaction (transaction.id)}
        <Transaction showAccount={accountId === undefined} {transaction} {tableStore}/>
    {/each}
</Table>
