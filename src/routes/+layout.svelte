<script lang="ts">
    import Sidebar from "../components/Sidebar.svelte";
	import "../styles/global.css";
	const { children } = $props();
    import { onMount } from "svelte";
    import { appStore } from "$lib/state.svelte";
    onMount(async ()=>{
        await appStore.load();
    });
</script>

<svelte:head>
    <link
      rel="stylesheet"
      type="text/css"
      href="https://cdn.jsdelivr.net/npm/@phosphor-icons/web@2.1.1/src/regular/style.css"
    />
    <link
      rel="stylesheet"
      type="text/css"
      href="https://cdn.jsdelivr.net/npm/@phosphor-icons/web@2.1.1/src/fill/style.css"
    />
</svelte:head>
<Sidebar/>
{@render children()}
