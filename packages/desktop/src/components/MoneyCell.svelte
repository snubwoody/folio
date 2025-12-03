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
    import { parseMoney } from "$lib/lib";
    import {TableCell} from "$components/table";

    type Props = {
        amount: string,
        symbol: string,
        onUpdate: (value:string) => void
    };

    const { symbol,amount,onUpdate }:Props = $props();

    let currentAmount = $derived(amount);
    async function updateAmount(){
        const newAmount = parseMoney(currentAmount);
        if (!newAmount){
            currentAmount = amount;
            return;
        }
        onUpdate(newAmount);
    }
</script>

<TableCell>
    <p>{symbol}</p>
    <input bind:value={currentAmount} onblur={updateAmount} class="outline-none">
</TableCell>
