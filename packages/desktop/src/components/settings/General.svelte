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
    import { getCurrencies } from "$lib/lib.js";
    import SelectMenu from "$components/SelectMenu.svelte";
    import { appStore } from "$lib/state.svelte";
    import Account from "./general/Account.svelte";
    import Support from "./support/Support.svelte";

    let currencies: string[] = $state([]);
    $effect(() => {
        getCurrencies()
            .then((c) => {
                currencies = c;
            });
    });

</script>

<div class="space-y-2">
    <div class="flex items-center justify-between w-full">
        <div class="space-y-0.5">
            <p>Currency code</p>
            <p class="text-text-muted text-sm">The ISO currency code</p>
        </div>
        <SelectMenu
            class="w-full max-w-12"
            defaultValue={appStore.settings.currencyCode}
            items={currencies}
            onChange={(c) => appStore.setCurrencyCode(c)}
            toOption={(item) => {
                return { label: item,value: item };
            }}
        />
    </div>
    <div class="w-full h-[1px] bg-neutral-50"></div>
    <section class="space-y-1">
        <h6>Accounts</h6>
        <ul class="space-y-2">
            {#each appStore.accounts as account (account.id)}
                <Account {account}/>
            {/each}
        </ul>
    </section>
</div>

