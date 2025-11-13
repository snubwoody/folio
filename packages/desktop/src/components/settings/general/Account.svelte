<!--
Copyright (C) 2025 Wakunguma Kalimukwa

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
-->
<script lang="ts">
    import { formatAmountWithoutSymbol, getCurrencySymbol  } from "$lib/lib.js";
    import { appStore } from "$lib/state.svelte";
    import IconButton from "$components/button/IconButton.svelte";
    import { Trash2 } from "@lucide/svelte";
    import type { Account } from "$lib/lib";
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
            onChange={(value) => appStore.accountStore.editAccount(account.id, { name: value })}
        />
        <div class="flex gap-0.5 items-center">
            <p class="text-sm">
                Starting balance: {symbol}
            </p>
            <InlineTextField
                class="text-sm"
                value={amount}
                onChange={(value) => appStore.accountStore.editAccount(account.id, { startingBalance: value })}
            />
        </div>
    </div>
    <IconButton aria-label="Delete account" onclick={() => appStore.deleteAccount(account.id)} variant="ghost">
        <Trash2/>
    </IconButton>
</li>