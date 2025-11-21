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
    import { Button } from "$components/button";
    import { TextArea } from "$components/input";
    import TextField from "$components/TextField.svelte";
    import { platform } from "@tauri-apps/plugin-os";
    import { getVersion } from '@tauri-apps/api/app';
    import {onMount} from "svelte";

    type FormState = "";
    let title = $state("Title");
    let description = $state("A detailed description of the issue...");
    // TODO: add error
    let form:HTMLFormElement;
    let currentPlatform = platform();

    let appVersion: string = $state("");

    const submit = (event: SubmitEvent) => {
        event.preventDefault();
    };

    onMount(async()=>{
        appVersion = await getVersion();
    })

</script>

<form bind:this={form} onsubmit={submit} class="flex flex-col gap-2.5">
    <div class="flex items-center justify-between">
        <div>App version</div>
        <div>v{appVersion}</div>
        <input type="hidden" name="version"  readonly value="1.2.2" />
    </div>
    <div class="flex items-center justify-between">
        <div>OS</div>
        <div>{currentPlatform}</div>
        <input type="hidden" name="os"  readonly value={currentPlatform} />
    </div>
    <TextField name="title" label="Title" bind:value={title}/>
    <TextArea name="description" class="max-w-[500px]" label="Description" bind:value={description}/>
    <Button class="w-full">Submit</Button>
</form>

