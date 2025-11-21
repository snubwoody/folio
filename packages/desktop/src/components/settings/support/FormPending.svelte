<script>
</script>

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


<div class="loader">
    <div class="circle"></div>
    <div class="circle"></div>
    <div class="circle"></div>
</div>

<style>
    .loader {
        display: flex;
        height: 100%;
        align-items: center;
        gap: 12px;
        justify-content: center;
        padding: 24px 0;
    }

    .circle{
        width: 12px;
        aspect-ratio: 1;
        border-radius: var(--radius-full);
        background: var(--color-neutral-800);
        animation: bounce 1s linear infinite;


        &:nth-child(2){
            animation-delay: 0.25s;
        }

        &:nth-child(3){
            animation-delay: 0.5s;
        }
    }

    @keyframes bounce{
        0%,100% {
            transform: translateY(0px);
        }

        50% {
            transform: translateY(-10px);
        }
    }
</style>

