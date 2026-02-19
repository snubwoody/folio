<script lang="ts">

</script>

<table>
    <thead>
        <tr>
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
        <tr>
            <td>12/12/12</td>
            <td>Account</td>
            <td>Payee</td>
            <td>Note</td>
            <td>Category</td>
            <td>$0.00</td>
            <td>$0.00</td>
        </tr>
        <tr>
            <td>12/12/12</td>
            <td>Account</td>
            <td>Payee</td>
            <td>Note</td>
            <td>Category</td>
            <td>$0.00</td>
            <td>$0.00</td>
        </tr>
        <tr>
            <td>12/12/12</td>
            <td>Account</td>
            <td>Payee</td>
            <td>Note</td>
            <td>Category</td>
            <td>$0.00</td>
            <td>$0.00</td>
        </tr>
    </tbody>
</table>

<style>
    table{
        table-layout: fixed;
        width: 100%;
    }

    td,th{
        text-align: left;

        &:last-child{
            text-align: right;
        }

        padding: 8px 16px;
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