<script lang="ts">
    import { Popover } from "bits-ui";
    import { TableCell } from "$components/table";
    import type { Transaction } from "$lib/api/transaction";
    import { formatMoney, parseMoney } from "$lib/utils/money";
    import type { HTMLAttributes } from "svelte/elements";

    interface Props extends HTMLAttributes<HTMLDivElement>  {
        transaction: Transaction;
        value?: string;
        onSubmit: (amount: string) => void;
    }

    const { transaction, value,onSubmit, ...rest }: Props = $props();

    let popupOpen = $state(false);
    let amount = $derived(formatMoney(value ?? "0", {stripSymbol: true}));

    const submit = () => {
        try {
            const num = parseFloat(amount);
            if (Number.isFinite(num)){
                onSubmit(amount);
            }
            popupOpen = false;
            return;
        } catch {

        }
    }
</script>

<TableCell {...rest}>
    <Popover.Root bind:open={popupOpen}>
        <Popover.Trigger class="text-left data-cell-padding w-full">
            {#if value}
                <p>{formatMoney(value)}</p>
            {:else}
                <p class="invisible">Select an item</p>
            {/if}
        </Popover.Trigger>
        <Popover.Portal>
            <Popover.Content
                class="popup-overlay rounded-sm"
                onCloseAutoFocus={(e) => e.preventDefault()}
                onInteractOutside={() => submit()}>
                    <form onsubmit={submit}>
                        <input
                            type="text"
                            bind:value={amount}
                            class="outline-none"
                        >
                        <input type="submit" hidden>
                    </form>
            </Popover.Content>
        </Popover.Portal>
    </Popover.Root>
</TableCell>
