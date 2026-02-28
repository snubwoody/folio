<script lang="ts">
    import { Toolbar } from "bits-ui";
    import { TextButton } from "$components/button";
    import { CirclePlus } from "@lucide/svelte";
    import { transactionStore } from "$lib/transaction.svelte";
    import { accountStore } from "$lib/account.svelte";
    import { today, getLocalTimeZone } from "@internationalized/date";

    async function addTransaction() {
        const account = accountStore.accounts[0];
        const date = today(getLocalTimeZone());
        await transactionStore.createExpense({
            amount: "0.0",
            date: date.toString(),
            account: account.id
        });
    }
</script>

<Toolbar.Root class="border-t border-neutral-50">
    <Toolbar.Group type="single">
        <Toolbar.GroupItem
            value="add-transaction"
            class="flex items-center px-2 py-1"
        >
            <TextButton theme="primary" onclick={addTransaction}>
                <CirclePlus />
                Add Transaction
            </TextButton>
        </Toolbar.GroupItem>
    </Toolbar.Group>
</Toolbar.Root>
