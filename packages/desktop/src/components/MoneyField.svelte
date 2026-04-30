<script lang="ts">
    import {settingsStore} from "$lib/stores/settings.svelte";
    // TODO: add more robust money input

    // TODO:
    // - Use the current currency to handle currencies
    // - Accessibility
    // - Arrow up and down
    // - Test input

    const currency  = $derived(settingsStore.currency)
    let value = $state<string|undefined>()

    const handleChange = (newValue: string) => {
        console.log(newValue);
        if (newValue.match(/^[0-9]$/)){
            value = newValue;
        } else{
            value = undefined;
        }
    }
</script>

<input type="text" bind:value onchange={(e) => handleChange(e.currentTarget.value)} placeholder="0.00">

<style>

</style>