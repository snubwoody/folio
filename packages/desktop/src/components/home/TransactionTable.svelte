<script lang="ts">
    import { TableStore } from "$lib/stores/table.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import Transaction from "./transaction/Transaction.svelte";

    interface Props {
        tableStore: TableStore
    }

    const { tableStore }: Props = $props();
    let selected = $derived(tableStore.allRowsSelected);
</script>

<table>
    <colgroup>
        <col class="w-7">
        <col>
        <col>
        <col>
        <col>
        <col>
        <col>
        <col>
    </colgroup>
    <thead>
        <tr>
            <th>
                <input type="checkbox" checked={selected} name="selected" id="row-checkbox"
                onclick={(e) => {
                    if(e.currentTarget.checked){
                        tableStore.toggleSelectAll();
                        return;
                    }
                    tableStore.toggleSelectAll();
                }}
                >
            </th>
            <th class="font-medium text-text-muted">Date</th>
            <th class="font-medium text-text-muted">Account</th>
            <th class="font-medium text-text-muted">Payee</th>
            <th class="font-medium text-text-muted">Note</th>
            <th class="font-medium text-text-muted">Category</th>
            <th class="font-medium text-text-muted">Outflow</th>
            <th class="font-medium text-text-muted">Inflow</th>
        </tr>
    </thead>
    <tbody>
        {#each transactionStore.transactions as transaction (transaction.id)}
            <Transaction {transaction} {tableStore}/>
        {/each}
    </tbody>
</table>

<style>
    table{
        table-layout: fixed;
        width: 100%;
        border-collapse: separate;
        border-spacing: 0;
    }

    th{
        text-align: left;

        /* &:last-child{
            text-align: right;
        } */

        padding: 8px 16px;
        border: 1px solid var(--color-neutral-50);
    }

    tr{
        border-bottom: 1px solid var(--color-neutral-50);

        &:first-child{
            border-top: 1px solid var(--color-neutral-50);
        }
    }
</style>
