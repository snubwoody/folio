<script lang="ts">
    import { TableStore } from "$lib/stores/table.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import Transaction from "./Transaction.svelte";
    import { Table, TableCell, TableHeader } from "$components/table";
    import { Checkbox } from "$components/select";

    interface Props {
        tableStore: TableStore,
        accountId?: string
    }

    const { tableStore,accountId }: Props = $props();
    let selected = $derived(tableStore.allRowsSelected);

    const select = (checked: boolean) => {
        if (checked){
            tableStore.selectAll();
            return;
        }
        tableStore.deselectAll();
    };

    let transactions = $derived.by(()=>{
        let transactions = transactionStore.transactions;

        if (accountId) {
            transactions = transactions.filter(t => t.fromAccountId === accountId);
        }

        return transactions;
    })

</script>

<Table length={accountId ? 7 : 8}>
    <TableHeader>
        <Checkbox bind:checked={selected} onChecked={select}/>
        <TableCell>Date</TableCell>
        {#if !accountId}
            <TableCell>Account</TableCell>
        {/if}
        <TableCell>Payee</TableCell>
        <TableCell>Note</TableCell>
        <TableCell>Category</TableCell>
        <TableCell>Outflow</TableCell>
        <TableCell>Inflow</TableCell>
    </TableHeader>
    {#each transactions as transaction (transaction.id)}
        <Transaction showAccount={accountId === undefined} {transaction} {tableStore}/>
    {/each}
</Table>

