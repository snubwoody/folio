<script lang="ts">
    import {Info,X} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {getVersion} from "@tauri-apps/api/app";
    import {Button, IconButton, TextButton} from "$components/button";

    let version = $state("");
    onMount(async()=>{
        version = await getVersion();
    })
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
        <Button>Check for updates</Button>
    </div>
    <div class="message-bar">
        <Info class="text-text-primary"/>
        <p class="w-full">A new update is available</p>
        <TextButton theme="primary" class="w-fit shrink-0">Download & Install</TextButton>
        <IconButton variant="ghost">
            <X/>
        </IconButton>
    </div>
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