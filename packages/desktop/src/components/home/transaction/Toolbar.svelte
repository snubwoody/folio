<script lang="ts">
    import { Toolbar } from "bits-ui";
    import { TextButton } from "$components/button";
    import { CirclePlus } from "@lucide/svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { accountStore } from "$lib/stores/account.svelte";

    async function addTransaction() {
        const account = accountStore.accounts[0];
        await transactionStore.createExpense({ accountId: account.id });
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
