<script lang="ts">
    import { formatAmount, formatAmountWithoutSymbol, getCurrencies, getCurrencySymbol  } from "$lib/lib.js";
    import { appStore } from "$lib/state.svelte";
    import IconButton from "$components/button/IconButton.svelte";
    import { Trash2 } from "@lucide/svelte";
    import type {Account} from "$lib/lib";
    import InlineTextField from "$components/InlineTextField.svelte";

    interface Props{
        account: Account
    }

    const { account }:Props = $props();
    const amount = $derived.by(() => formatAmountWithoutSymbol(account.startingBalance));
    const symbol = getCurrencySymbol(appStore.settings.currencyCode);
</script>

<li class="flex items-center justify-between">
    <div class="space-y-0.5">
        <InlineTextField 
            value={account.name}
            onChange={(value) => appStore.accountStore.editAccount(account.id, {name: value})}
        />
        <div class="flex gap-0.5 items-center">
            <p class="text-sm">
                Starting balance: {symbol}
            </p>
            <InlineTextField 
                class="text-sm" 
                value={amount}
                onChange={(value) => appStore.accountStore.editAccount(account.id, {startingBalance: value})}
            />
        </div>
    </div>
    <IconButton aria-label="Delete account" onclick={() => appStore.deleteAccount(account.id)} variant="ghost">
        <Trash2/>
    </IconButton>
</li>