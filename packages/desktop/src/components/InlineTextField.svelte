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
    interface Props {
        onChange?: (value:string) => void,
        value?: string,
        class?: string,
    };

    const { onChange, value, class: userClass }: Props = $props();

    let inner = $state(value ?? "");

    const update = () => {
        onChange?.(inner);
    };
</script>

<label class="inline-text-field">
    <input
        type="text"
        bind:value={inner}
        onblur={update}
        class={`${userClass}`}
    />
</label>

<style>
    .inline-text-field {
        position: relative;

        &::after {
            position: absolute;
            content: "";
            height: 1px;
            bottom: -4px;
            left: 0;
            width: 100%;
            background-color: transparent;
            transition: all 250ms;
        }

        &:focus-within {
            &::after {
                background-color: black;
            }
        }

        input {
            outline: none;
        }
    }
</style>
