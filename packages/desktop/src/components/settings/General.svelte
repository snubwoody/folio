<script lang="ts">
    import {formatAmount, getCurrencies} from "$lib/lib.js";
    import SelectMenu from "$components/SelectMenu.svelte";
    import { appStore } from "$lib/state.svelte";
    import IconButton from "$components/button/IconButton.svelte";
    import {Trash2} from "@lucide/svelte";

    let currencies: string[] = $state([]);
    $effect(()=>{
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
            class="w-full max-w-[96px]"
            defaultValue={appStore.settings.currencyCode}
            items={currencies}
            onChange={(c) => appStore.setCurrencyCode(c)}
            toOption={(item) => {return { label: item,value: item };}}
        />
    </div>
    <section class="space-y-1">
        <h6>Accounts</h6>
        <ul class="space-y-2">
            {#each appStore.accounts as account}
                <li class="flex items-center justify-between">
                    <div class="space-y-0.5">
                        <p>{account.name}</p>
                        <p class="text-sm">
                            Starting balance: {formatAmount(account.startingBalance,{currency:appStore.settings.currencyCode})}
                        </p>
                    </div>
                    <IconButton variant="ghost">
                        <Trash2/>
                    </IconButton>
                </li>
            {/each}
        </ul>
    </section>
</div>

