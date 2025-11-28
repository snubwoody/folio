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
    import Sidebar from "../components/Sidebar.svelte";
    import Titlebar from "$components/Titlebar.svelte";
    import ToastGroup from "$components/popups/ToastGroup.svelte";
    import "../styles/global.css";
    import { onMount } from "svelte";
    import { appStore } from "$lib/state.svelte";

    const { children } = $props();

    onMount(async () => {
        await appStore.load();
    });
</script>

<!---FIXME: remove icon url-->
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

<Titlebar />
<main class="flex">
    <Sidebar />
    {@render children()}
    <ToastGroup/>
</main>

<style>
    :global(body) {
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    main {
        display: grid;
        grid-template-columns: auto 1fr;
        overflow-y: hidden;
        height: 100%;
    }
</style>
