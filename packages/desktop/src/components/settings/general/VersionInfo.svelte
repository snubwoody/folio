<script lang="ts">
    import {Info,X} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {getVersion} from "@tauri-apps/api/app";
    import {Button, IconButton, TextButton} from "$components/button";
    import {checkForUpdate,installUpdate} from "$lib/utils/update";
    import type {Update} from "@tauri-apps/plugin-updater";
    import { MessageBar } from "$components/alerts";

    let version = $state("");
    onMount(async()=>{
        version = await getVersion();
    })

    // TODO: omit check for update if not updatable, include in dev?
    let updatePending = $state(false);
    let update = $state<Update | null>(null)
    export const checkUpdate = async () => {
        updatePending = false;
        update = await checkForUpdate();
    }
</script>

<section class="space-y-2.5">
    <div class="flex items-center justify-between">
        <div class="space-y-0.5">
            <div class="flex items-center gap-0.5">
                <h6>Version</h6>
                <h6>{version}</h6>
            </div>
            <a href="https://github.com/snubwoody/folio/blob/main/CHANGELOG.md" target="_blank" class="text-text-primary underline text-sm">
                Read the changelog
            </a>
        </div>
        <Button onclick={checkUpdate}>Check for updates</Button>
    </div>
    <MessageBar message="No update found"/>
    <MessageBar message="A new update is available">
        {#if updatePending}
            <p>Checking for update...</p>
        {:else}
            <TextButton theme="primary" class="w-fit shrink-0">
                Download & Install
            </TextButton>
        {/if}
    </MessageBar>
</section>

<style>
    .message-bar{
        display: flex;
        border: 1px solid var(--color-border-neutral-10);
        border-radius: var(--radius-md);
        padding: 12px;
        gap: 12px;
        align-items: center;
    }
</style>