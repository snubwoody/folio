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
    import {createDialog} from "@melt-ui/svelte";
    import SettingsPanel from "$components/settings/SettingsPanel.svelte";

    const {
        elements: {trigger,portalled,overlay,content,},
        states: {open}
    }= createDialog();
</script>

<div {...$trigger} use:trigger class="mt-auto">
    <IconButton variant="neutral" class="mt-auto">
        <Settings size="20" class="w-3"/>
    </IconButton>
</div>

{#if $open}
    <div {...$portalled} use:portalled>
        <div {...$overlay} use:overlay></div>
        <div {...$content} use:content>
            <SettingsPanel/>
        </div>
    </div>
{/if}

<style>
    @keyframes fade-in{
        from{
            opacity: 0;
        }
    }
    [data-melt-dialog-portalled] {
        position: fixed;
        inset: 0;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    [data-melt-dialog-overlay]{
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.1);
        animation: fade-in 250ms linear;
    }

    [data-melt-dialog-content]{
        width: 70vw;
        height: 70vh;
        background: var(--color-white);
        z-index: 200;
        border-radius: var(--radius-lg);
        box-shadow: var(--shadow-md);
    }
</style>
