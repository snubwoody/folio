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
    import Settings from "@lucide/svelte/icons/settings";
    import IconButton from "$components/button/IconButton.svelte";
    // import { createDialog } from "@melt-ui/svelte";
    import { Dialog } from "bits-ui";
    import SettingsPanel from "$components/settings/SettingsPanel.svelte";

</script>

<Dialog.Root>
    <Dialog.Trigger class="mt-auto" aria-label="Open settings">
        <IconButton variant="ghost" class="mt-auto">
            <Settings size="20" class="w-3"/>
        </IconButton>
    </Dialog.Trigger>
    <Dialog.Portal>
        <Dialog.Content class="settings-portal">
            <SettingsPanel/>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>

<style>
    @keyframes fade-in{
        from{
            opacity: 0;
        }
    }

    :global(.settings-portal){
        position: fixed;
        inset: 6rem 12rem;
        background: var(--color-white);
        z-index: 200;
        border-radius: var(--radius-lg);
        box-shadow: var(--shadow-md);
        overflow: hidden;
    }
</style>
