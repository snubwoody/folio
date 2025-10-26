<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import square from "$assets/fluent_square.svg";
    import dismiss from "$assets/fluent_dismiss.svg";
    import subtract from "$assets/fluent_subtract.svg";
    import squareMultiple from "$assets/fluent_square_multiple.svg";
    import { platform } from "@tauri-apps/plugin-os";
    const window = getCurrentWindow();
    let maximised = $state(true);
    const currentPlatform = platform();

    $effect(() => {
        window.onResized(() => {
            window.isMaximized().then((val) => (maximised = val));
        });
    });
</script>

<!--- Use system defaults on all other platforms -->
{#if currentPlatform === "windows"}
    <div class="title-bar">
        <div
            aria-hidden="true"
            class="w-full"
            onmousedown={() => window.startDragging()}
        >
            <p class="mr-auto">Folio</p>
        </div>
        <button
            id="minimize-btn"
            aria-label="Minimise"
            onclick={() => window.minimize()}
        >
            <img src={subtract} alt="" />
        </button>
        <button
            id="maximize-btn"
            aria-label="Maximize"
            onclick={() => {
                window.toggleMaximize();
            }}
        >
            {#if maximised}
                <img src={squareMultiple} alt="" />
            {:else}
                <img src={square} alt="" />
            {/if}
        </button>
        <button
            id="close-btn"
            aria-label="Close"
            onclick={() => window.close()}
        >
            <img src={dismiss} alt="" />
        </button>
    </div>
{/if}

<style lang="ts">
    .title-bar {
        display: flex;
        align-items: center;
        padding-left: 12px;
        background-color: var(--color-neutral-25);
        border-bottom: 1px solid var(--color-neutral-100);
    }

    button {
        padding: 12px;
        &:hover {
            background: var(--color-neutral-50);
        }
    }
    img {
        width: 16px;
        height: 16px;
    }
    #close-btn {
        &:hover {
            background: #e81123;
            color: white;
        }
    }
</style>
