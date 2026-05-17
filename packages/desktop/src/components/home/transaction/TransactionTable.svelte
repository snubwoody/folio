<script lang="ts">
    import { TableStore } from "$lib/stores/table.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import Transaction from "./Transaction.svelte";
    import { Table, TableCell, TableHeader } from "$components/table";
    import { Checkbox } from "$components/select";

    interface Props {
        tableStore: TableStore
    }

    const { tableStore }: Props = $props();
    let selected = $derived(tableStore.allRowsSelected);
    const select = (checked: boolean) => {
        if (checked){
            tableStore.selectAll();
            return;
        }
        tableStore.deselectAll();
    };
</script>

<Table length={8}>
    <TableHeader>
        <Checkbox bind:checked={selected} onChecked={select}/>
        <TableCell>Date</TableCell>
        <TableCell>Account</TableCell>
        <TableCell>Payee</TableCell>
        <TableCell>Note</TableCell>
        <TableCell>Category</TableCell>
        <TableCell>Outflow</TableCell>
        <TableCell>Inflow</TableCell>
    </TableHeader>
    {#each transactionStore.transactions as transaction (transaction.id)}
        <Transaction {transaction} {tableStore}/>
    {/each}
</Table>

