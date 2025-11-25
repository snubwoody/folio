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
    import type { HTMLButtonAttributes } from "svelte/elements";
    import type { Snippet } from "svelte";
    type Variant = "primary" | "secondary" | "neutral" | "bordered" | "ghost";
    type Size = "small" | "medium";

    interface Props extends HTMLButtonAttributes{
        variant?: Variant,
        size?: Size,
        children: Snippet
    }

    const {
        variant = "primary",
        size = "medium",
        children,
        class:userClass,
        ...rest
    }: Props = $props();
</script>

<button class={`btn btn-${variant} btn-${size} ${userClass}`} {...rest}>
    {@render children()}
</button>

