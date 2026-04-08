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
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { accountStore } from "$lib/stores/account.svelte";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { check, type Update } from "@tauri-apps/plugin-updater";
    import { relaunch } from "@tauri-apps/plugin-process";
    import { addToast } from "$lib/toast.svelte";
    import { logger } from "$lib/logger";
    import { BundleType, getBundleType } from "@tauri-apps/api/app";

    const { children } = $props();

    async function installUpdate(update: Update){
        logger.info(`Downloading new update (${update.version})`);
        await update.downloadAndInstall().catch(err => logger.error(`Failed to install new update: ${err.message}`));
        logger.info(`Installed update (${update.version}), relaunching app...`);
        await relaunch();
    }

    async function checkForUpdate(){
        const bundleType = await getBundleType();
        const updatableBundles = [BundleType.AppImage,BundleType.Nsis,BundleType.App];
        if (!updatableBundles.includes(bundleType)) return;

        const update = await check();
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
        await invoke("create_missing_budgets");
        await settingsStore.load();
        await appStore.load();
        await transactionStore.load();
        await accountStore.load();
        await categoryStore.load();
        checkForUpdate();
    });

    $effect(() => {
        transactionStore.sort();
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
