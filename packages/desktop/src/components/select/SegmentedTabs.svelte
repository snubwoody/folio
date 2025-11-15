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
<!--
@component
# Segmented tabs
A tab bar that display a list of tab items to the user, each corresponding to
a specific tab content.

# Example
```svelte
<script>
    import { SegmentedTabs,TabContent,TabButton } from "$components/select";
    let value = "Tab 1";
</script>

<SegmentedTabs bind:value>
    <TabButton value="Tab 1">Tab 1</TabButton>
    <TabButton value="Tab 2">Tab 2</TabButton>
    <TabContent value="Tab 1">Tab 1</TabContent>
    <TabContent value="Tab 2">Tab 2</TabContent>
</SegmentedTabs>
```
-->
<script lang="ts">
    import { Tabs } from "bits-ui";
    import type { Snippet } from "svelte";
    import "$styles/tabs.css";

    type SegmentedTabStyle = "primary" | "neutral";

    interface Props{
        value: string,
        children: Snippet,
        variant?: SegmentedTabStyle,
    }

    let tabRoot = $state<HTMLDivElement | null>(null);
    let {
        value = $bindable(),
        variant = "primary",
        children
    }: Props = $props();

    $effect(() => {
        // eslint-disable-next-line @typescript-eslint/no-unused-expressions
        value;
        movePill();
    });

    function movePill(){
        if (!tabRoot){
            return;
        }
        const activeElement = tabRoot?.querySelector("[data-tabs-trigger][data-state='active']");
        if (!activeElement) return;
        const rect = activeElement.getBoundingClientRect();
        const offset = activeElement?.getBoundingClientRect().x - tabRoot.getBoundingClientRect().x;
        let pill = tabRoot?.querySelector<HTMLElement>(".segmented-tabs-pill");
        if (!pill) return;
        pill.style.transform = `translatex(${offset}px)`;
        pill.style.width = `${rect.width}px`;
        pill.style.height = `${rect.height}px`;
    }

    function getValue(): string {
        return value;
    }

    function setValue(newValue: string){
        value = newValue;
    }
</script>

<Tabs.Root bind:ref={tabRoot} class={`segmented-tabs segmented-tabs-${variant}`} bind:value={getValue,setValue}>
    {@render children()}
</Tabs.Root>

