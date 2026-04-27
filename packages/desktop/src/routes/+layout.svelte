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
    import Sidebar from "$components/sidebar/Sidebar.svelte";
    import Titlebar from "$components/Titlebar.svelte";
    import ToastGroup from "$components/alerts/ToastGroup.svelte";
    import "$styles/global.css";
    import { onMount } from "svelte";
    import { appStore } from "$lib/state.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { accountStore } from "$lib/stores/account.svelte";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { checkForUpdate, installUpdate } from "$lib/utils/update";
    import { addToast } from "$lib/stores/toast.svelte";

    const { children } = $props();

    async function updateApp(){
        const update = await checkForUpdate();
        if (update){
            addToast({
                title: "A new update is available",
                primaryAction:{
                    text: "Download and install",
                    action: () => installUpdate(update)
                }
            });
        }

    }

    onMount(async () => {
        await Promise.all([
            invoke("create_missing_budgets"),
            appStore.load(),
            transactionStore.load(),
            accountStore.load(),
            categoryStore.load(),
            settingsStore.load(),
            updateApp()
        ]);
        transactionStore.sort();
    });
</script>

<Titlebar />
<div>
    <Sidebar/>
    {@render children()}
    <ToastGroup/>
</div>

<style>
    :global(body) {
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    div {
        display: flex;
        overflow-y: hidden;
        height: 100%;
    }
</style>
