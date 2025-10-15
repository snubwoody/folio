<script lang="ts">
    import Sidebar from "../components/Sidebar.svelte";
	import "../styles/global.css";
	const { children } = $props();
    import { onMount } from "svelte";
    import { accountStore } from "$lib/account.svelte";
    import { transactionStore } from "$lib/transaction.svelte";
    import { analyticsStore } from "$lib/analytics.svelte";
    onMount(async ()=>{
        await accountStore.load();
        await transactionStore.load();
        await analyticsStore.load();
    });

    $effect(()=>{
        // Make eslint ignore this
        void transactionStore.expenses;
        analyticsStore.load();
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
