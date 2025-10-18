<script lang="ts">
    import { parseMoney } from "$lib/lib";

    type Props = {
        amount: string,
        symbol: string,
        onUpdate: (value:string) => void
    }

    let {symbol,amount,onUpdate}:Props = $props();

    let currentAmount = $derived(amount);
    async function updateAmount(){
        try{
            const newAmount = parseMoney(currentAmount);
            if (!newAmount){
                throw "";
            }
            onUpdate(newAmount);
        } catch(e){
            currentAmount = amount;
        }
    }
</script>

<div class="data-cell flex items-center gap-0.5">
    <p>{symbol}</p>
    <input bind:value={currentAmount} onblur={updateAmount} class="outline-none">
</div>
