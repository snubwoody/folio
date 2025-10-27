<script lang="ts">
    import TextField from "$components/TextField.svelte";
    import {getCurrencies} from "$lib/lib.js";
    import SelectMenu from "$components/SelectMenu.svelte";
    import {appStore} from "$lib/state.svelte";

    let currencies: string[] = $state([]);
    $effect(()=>{
        getCurrencies()
            .then((c) => {
                currencies = c;
            });
    })
</script>

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
        toOption={(item) => {return {label: item,value: item};}}
    />
</div>
