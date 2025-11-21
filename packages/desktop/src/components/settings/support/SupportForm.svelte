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
    import Form from "./Form.svelte";
    import { SegmentedTabs,TabContent,TabButton,TabBar } from "$components/select";
    import type {SupportResponse} from "$lib/lib";
    import FormSuccess from "$components/settings/support/FormSuccess.svelte";
    import FormError from "$components/settings/support/FormError.svelte";
    import FormPending from "$components/settings/support/FormPending.svelte";

    type TabType = "feature" | "bug";
    type FormState = "default" | "pending" | "success" | "error";
    let activeTab = $state<TabType>("feature");
    let formState = $state<FormState>("default");
    let response: SupportResponse = {
        issueId: 14,
        issueUrl: "https://github.com"
    };
</script>

{#if formState === "default"}
    <SegmentedTabs variant="neutral" bind:value={activeTab}>
        <TabBar class="w-full">
            <TabButton value="feature">Feature request</TabButton>
            <TabButton value="bug">Bug report</TabButton>
        </TabBar>
        <TabContent value="feature">
            <Form/>
        </TabContent>
        <TabContent value="bug">
            <Form/>
        </TabContent>
    </SegmentedTabs>
{:else if formState === "pending"}
    <FormPending/>
{:else if formState === "success"}
    <FormSuccess {response}/>
{:else if formState === "error"}
    <FormError/>
{/if}

