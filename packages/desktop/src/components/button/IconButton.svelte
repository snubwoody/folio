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
    import type { Snippet } from "svelte";
    import "$styles/icon-btn.css";
    import type { HTMLButtonAttributes } from "svelte/elements";

    type Style = "primary" | "ghost" | "neutral";
    type Size = "small" | "medium";

    type Props = {
        children: Snippet
        variant?: Style
        size?: Size,
        href?: string,
        class?: string
    } & HTMLButtonAttributes;

    const {
        children,
        size = "medium",
        variant = "primary",
        href,
        class:userClass,
        ...rest
    }: Props = $props();
</script>

{#if href}
    <a {href} class={`icon-btn icon-btn-${variant} icon-btn-${size} ${userClass}`}>
        {@render children()}
    </a>
{:else}
    <button class={`icon-btn icon-btn-${variant} icon-btn-${size} ${userClass}`} {...rest}>
        {@render children()}
    </button>
{/if}

