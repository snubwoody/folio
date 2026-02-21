<script lang="ts">
    import { TableStore } from "$lib/stores/table.svelte";
    import { transactionStore } from "$lib/transaction.svelte";
    import { setContext } from "svelte";
    import Transaction from "./Transaction.svelte";
    const tableStore = new TableStore();

    setContext("tableStore",tableStore);
</script>

<table>
    <colgroup>
        <col class="w-8">
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
                <input type="checkbox" name="selected" id="row-checkbox"
                onclick={(e)=>{
                    if(e.currentTarget.checked){
                        tableStore.toggleSelectAll();
                        return
                    }
                    tableStore.toggleSelectAll();
                }}
                >
            </th>
            <th>Date</th>
            <th>Account</th>
            <th>Payee</th>
            <th>Note</th>
            <th>Category</th>
            <th>Outflow</th>
            <th>Inflow</th>
        </tr>
    </thead>
    <tbody>
        {#each transactionStore.transactions as transaction (transaction.id)}
            <Transaction {transaction}/>
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

        &:last-child{
            text-align: right;
        }

        padding: 8px 16px;
        border: 1px solid var(--color-neutral-50);
    }

    thead{
        background: var(--color-neutral-25);
    }

    tr{
        border-bottom: 1px solid var(--color-neutral-50);

        &:first-child{
            border-top: 1px solid var(--color-neutral-50);
        }
    }
</style>