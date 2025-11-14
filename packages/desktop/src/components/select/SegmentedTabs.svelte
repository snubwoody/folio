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
A tab bar that display a list of tab items to the user, each corresponding to
a specific tab content.

# Example
```svelte
<script>
    const paymentMethods = [
        {id: 298024, title: "Apple Pay"},
        {id: 232353, title: "Klarna"},
        {id: 368547, title: "Bank"},
        {id: 869822, title: "Credit card"},
        {id: 724024, title: "Cash"},
        {id: 239142, title: "PayPal"},
    ]
</script>

<SelectMenu
    label = "Payment method"
    items={paymentMethods}
    toOption = {(item) => {value: item.id, label: item.title}}
    onChange = {(item) => console.log(item)}
/>
```
-->
<script lang="ts">
    import {Tabs} from "bits-ui";
    import type {Snippet} from "svelte";

    interface Props{
        value: string,
        children: Snippet
    }

    let {
        value = $bindable(),
        children
    }: Props = $props();
</script>

<Tabs.Root bind:value>
    {@render children()}
</Tabs.Root>
