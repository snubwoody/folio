<script lang="ts">
    import { onMount } from "svelte";
    import { getVersion } from "@tauri-apps/api/app";
    import { Button, TextButton } from "$components/button";
    import { checkForUpdate, installUpdate } from "$lib/utils/update";
    import type { Update } from "@tauri-apps/plugin-updater";
    import { MessageBar } from "$components/alerts";

    let version = $state("");
    onMount(async() => {
        version = await getVersion();
    });

    let updatePending = $state(false);
    let update = $state<Update | null>(null);
    let noUpdateFound = $state(false);

    const checkUpdate = async () => {
        updatePending = false;
        update = await checkForUpdate();
        if (!update){
            noUpdateFound = true;
            // eslint-disable-next-line no-undef
            setTimeout(() => noUpdateFound = false,3500);
        }
    };

    const updateApp = async () => {
        if (!update){
            return;
        }
        updatePending = true;
        await installUpdate(update);
    };
</script>

<section class="space-y-2.5">
    <div class="flex items-center justify-between">
        <div>
            <div class="flex items-center gap-0.5">
                <h6>Version</h6>
                <h6>{version}</h6>
            </div>
            <a href="https://github.com/snubwoody/folio/releases/latest" target="_blank" class="text-text-primary underline text-sm">
                Read the changelog
            </a>
        </div>
        <Button onclick={checkUpdate}>Check for updates</Button>
    </div>
    {#if noUpdateFound}
        <MessageBar message="No update found" bind:open={noUpdateFound}/>
    {/if}
    {#if update}
        <MessageBar message="A new update is available">
            {#if updatePending}
                <p class="shrink-0">Installing update...</p>
            {:else}
                <TextButton theme="primary" class="w-fit shrink-0" onclick={updateApp}>
                    Download & Install
                </TextButton>
            {/if}
        </MessageBar>
    {/if}
</section>
