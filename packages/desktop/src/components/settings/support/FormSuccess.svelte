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
    import type {SupportResponse} from "$lib/lib";
    import {Button} from "$components/button";

    interface Props{
        response: SupportResponse
    }

    const {response}: Props = $props();
</script>

<div>
    <p>Your bug report has been submitted please see the progress at the link below:</p>
    <a href={response.issueUrl}>Issue: {response.issueId}</a>
    <Button class="w-full" variant="neutral">Close</Button>
</div>

<style>
    div{
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 24px;
    }

    P{
        text-align: center;
    }

    a{
        color: var(--color-purple-500);
    }
</style>