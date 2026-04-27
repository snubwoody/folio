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
    import SelectMenu from "$components/SelectMenu.svelte";
    import { accountStore } from "$lib/stores/account.svelte";
    import Account from "./Account.svelte";
    import VersionInfo from "./VersionInfo.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { getCurrencies } from "$lib/utils/money";
    import type { Currency } from "$lib/types";

    let currencies: Currency[] = $state([]);
    $effect(() => {
        getCurrencies()
            .then((c) => {
                if (c){
                    currencies = c;
                }
            });
    });

</script>

<div class="space-y-2">
    <VersionInfo/>
    <div class="w-full h-px bg-neutral-50"></div>
    <section class="flex items-center justify-between w-full">
        <div class="space-y-0.5">
            <p>Currency code</p>
            <p class="text-text-muted text-sm">The ISO currency code</p>
        </div>
        <SelectMenu
            class="w-fit"
            items={currencies}
            onChange={(c) => settingsStore.setCurrencyCode(c.code)}
            toOption={(item) => {
                return { label: `${item.name} - ${item.symbol ?? item.code}`,value: item.code };
            }}
        />
    </section>
    <div class="w-full h-px bg-neutral-50"></div>
    <section class="space-y-1">
        <h6>Accounts</h6>
        <ul class="space-y-2">
            {#each accountStore.accounts as account (account.id)}
                <Account {account}/>
            {/each}
        </ul>
    </section>
</div>

