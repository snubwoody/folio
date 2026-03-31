<script lang="ts">
import data from "@emoji-mart/data";
import { Picker } from "emoji-mart";


const pickerOptions = { onEmojiSelect: console.log,data };
const picker = new Picker({data});
// document.body.append(picker);

let div: HTMLDivElement;

$effect(()=>{
    div.appendChild(picker);
});
</script>

<div bind:this={div}></div>
