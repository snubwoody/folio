<script lang="ts">
    import { formatAmount  } from "$lib/lib.js";
    import { appStore } from "$lib/state.svelte";
    import IconButton from "$components/button/IconButton.svelte";
    import { Trash2 } from "@lucide/svelte";
    import type {Account} from "$lib/lib";

    interface Props{
        account: Account
    }

    const { account }:Props = $props();
</script>

<li class="flex items-center justify-between">
    <div class="space-y-0.5">
        <p>{account.name}</p>
        <p class="text-sm">
            Starting balance: {formatAmount(account.startingBalance,{ currency:appStore.settings.currencyCode })}
        </p>
    </div>
    <IconButton aria-label="Delete account" onclick={() => appStore.deleteAccount(account.id)} variant="ghost">
        <Trash2/>
    </IconButton>
</li>