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
    import { toastStore } from "$lib/toast.svelte.js";
    import {Info,X} from "@lucide/svelte";
    import { IconButton, Button } from "$components/button";
    import { fly } from "svelte/transition";
    // TODO: close toast on esc
</script>

<div class="toast-group">
    {#each toastStore.toasts as toast}
        <div transition:fly={{x:"50"}} class="toast">
            <div class="flex gap-1">
                <div class="py-0.5">
                    <Info size="20" class="text-purple-500"/>
                </div>
                <div>
                    <p>{toast.title}</p>
                    {#if toast.body}
                        <p class="text-sm text-text-muted">A new update is available.</p>
                    {/if}
                </div>
                <IconButton variant="ghost" class="ml-auto" onclick={() => toastStore.removeToast(toast.id)}>
                    <X size="16"/>
                </IconButton>
            </div>
            {#if toast.secondaryAction || toast.primaryAction}
                <div class="flex justify-end items-center gap-1.5">
                    {#if toast.secondaryAction}
                        <Button size="small" variant="ghost" onclick={toast.secondaryAction.action}>
                            {toast.secondaryAction.text}
                        </Button>
                    {/if}
                    {#if toast.primaryAction}
                        <Button size="small"  onclick={toast.primaryAction.action}>
                            {toast.primaryAction.text}
                        </Button>
                    {/if}
                </div>
            {/if}
        </div>
    {/each}
</div>

<style>
    .toast-group{
        position: fixed;
        display: flex;
        padding: 20px 40px;
        flex-direction: column-reverse;
        gap: 16px;
        bottom: 0;
        right: 0;
    }

    .toast{
        width: 100vw;
        max-width: 350px;
        display: flex;
        flex-direction: column;
        gap: 16px;
        padding: 16px;
        box-shadow: var(--shadow-md);
        border-radius: var(--radius-md);
        border: 1px solid var(--color-border-neutral-10);
    }
</style>