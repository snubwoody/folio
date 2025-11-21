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
    import { Button, IconButton } from "$components/button";
    import { Dialog } from "bits-ui";
    import SupportForm from "./SupportForm.svelte";
    import { X } from "@lucide/svelte";
</script>

<section>
    <h6>Support</h6>
    <div class="flex items-center justify-between">
        <div>
            <p>Submit a bug report/feature request</p>
            <p class="text-text-muted">
                You can check the existing issues or feature requests to see if the bug already exists.
            </p>
        </div>
        <Dialog.Root>
            <Dialog.Trigger >
                <Button variant="neutral">Open form</Button>
            </Dialog.Trigger>
            <Dialog.Portal>
                <Dialog.Overlay />
                <Dialog.Content class="support-form">
                    <div class="flex flex-col gap-3">
                        <div class="flex justify-between items-center">
                            <Dialog.Title>
                                <h6>Support form</h6>
                            </Dialog.Title>
                            <Dialog.Close>
                                <IconButton variant="ghost"><X size="20"/></IconButton>
                            </Dialog.Close>
                        </div>
                        <SupportForm/>
                    </div>
                </Dialog.Content>
            </Dialog.Portal>
        </Dialog.Root>
    </div>
</section>

<style>
    :global(.support-form){
        position: fixed;
        width: 100vw;
        max-width: 500px;
        left: 50%;
        top: 50%;
        padding: 24px;
        translate: -50% -50%;
        background: var(--color-white);
        z-index: 200;
        border-radius: var(--radius-lg);
        box-shadow: var(--shadow-md);
        overflow: hidden;
        height: fit-content;
        transition: all 250ms ease;
    }
</style>