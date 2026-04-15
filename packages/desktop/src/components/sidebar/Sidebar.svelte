
<script>
    import IconButton from "$components/button/IconButton.svelte";
    import SettingsButton from "$components/settings/SettingsButton.svelte";
    import { Landmark,ChartNoAxesCombined,PanelRightOpen,PanelRightClose } from "@lucide/svelte";
    import { page } from "$app/state";
    import AccountSection from "./AccountSection.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";

    let expanded = $derived(settingsStore.settings.sidebarOpen);

    const toggleSidebar = () => {
        settingsStore.setSidebarState(!expanded);
    };
</script>

<aside id="nav-panel" data-testid="nav-panel" data-expanded={expanded}>
    <div class="flex flex-col">
        <a href="/" class="page-link" data-selected={page.route.id === "/"}>
            <Landmark/>
            <p>Transactions</p>
        </a>
        <a href="/analytics" class="page-link" data-selected={page.route.id === "/analytics"}>
            <ChartNoAxesCombined/>
            <p>Spending</p>
        </a>
    </div>
    <div class="h-[3px] w-full bg-neutral-100"></div>
    <AccountSection/>
    <!---TODO: check border-->
    <!---TODO: change icon-->
    <div class="nav-panel-actions">
        <SettingsButton/>
        <IconButton
            variant="ghost"
            aria-label="Collapse sidebar"
            aria-expanded={expanded}
            aria-controls="nav-panel"
            onclick={toggleSidebar}
        >
            {#if expanded}
                <PanelRightOpen/>
            {:else}
                <PanelRightClose/>
            {/if}
        </IconButton>
    </div>
</aside>

<style>
	aside{
		width: 100%;
		height: 100%;
        max-width: 350px;
		display: flex;
		flex-direction: column;
        gap: 24px;
        padding-top: 12px;
		background-color: var(--color-neutral-25);
        border-right: 1px solid var(--color-neutral-100);
        transition: all 250ms;

        &:global([data-expanded="false"]){
            width: fit-content;
        }
	}

    .page-link{
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 8px 16px;
        transition: all 250ms;
        box-sizing: content-box;

        :global(svg){
            width: 16px;
            height: 16px;
        }

        p{
            :global([data-expanded="false"]) &{
                display: none;
            }
        }

        &:hover{
            background-color: var(--color-purple-50);
        }

        &:global([data-selected="true"]){
            color: var(--color-text-primary);
            box-shadow: inset 4px 0 0 0 var(--color-surface-primary);
        }
    }

    .nav-panel-actions{
        display: flex;
        margin-top: auto;
        border-top: 1px solid var(--color-neutral-100);
        align-items: center;
        padding-block: 12px;

        :global([data-expanded="false"]) &{
            flex-direction: column-reverse;
        }
    }
</style>
