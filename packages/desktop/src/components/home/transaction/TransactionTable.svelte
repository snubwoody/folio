<script lang="ts">
    import { TableStore } from "$lib/stores/table.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import Transaction from "./Transaction.svelte";
    import { Table, TableCell, TableHeader } from "$components/table";

    interface Props {
        tableStore: TableStore
    }

    const { tableStore }: Props = $props();
    let selected = $derived(tableStore.allRowsSelected);
</script>
<Table>
    <TableHeader>
        <input type="checkbox" checked={selected} class="w-fit" name="selected" id="row-checkbox"
           onclick={(e) => {
               if(e.currentTarget.checked){
                   tableStore.toggleSelectAll();
                   return;
               }
               tableStore.toggleSelectAll();
           }}
        >
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

