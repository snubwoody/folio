
<script>
    import IconButton from "$components/button/IconButton.svelte";
    import Home from "@lucide/svelte/icons/house";
    import Chart from "@lucide/svelte/icons/chart-no-axes-combined";
    import SettingsButton from "$components/settings/SettingsButton.svelte";
    import {Plus,Landmark,ChartNoAxesCombined,PanelRightOpen,PanelRightClose} from "@lucide/svelte";
    import {Button} from "$components/button";
    import {accountStore} from "$lib/stores/account.svelte";
    import {formatMoney} from "$lib/lib";
    // TODO: add aria-expanded

    const total = $derived.by(()=>{
        // TODO: test 0 accounts
        const t = accountStore.accounts.map(a => parseFloat(a.balance)).reduce((prev, current) => current + prev,0);
        return t.toString();
    });

    // TODO: max-height for accounts
    let expanded = $state(true);
</script>

<aside id="nav-panel" data-expanded={expanded}>
    <div class="flex flex-col">
        <a href="/" class="page-link">
            <Landmark/>
            <p>Transactions</p>
        </a>
        <a href="/analytics" class="page-link">
            <ChartNoAxesCombined/>
            <p>Spending</p>
        </a>
    </div>
    <div class="h-[3px] w-full bg-neutral-100"></div>
    <ul class="account-list">
        <li class="font-medium">
            <p>Accounts</p>
            <p>{formatMoney(total)}</p>
        </li>

        <!--TODO: test these and test total-->
        {#each accountStore.accounts as account (account.id)}
            <!--TODO: add title-->
            <!--FIXME: use accountBalance method-->
            <li>
                <p class="text-truncate max-w-[50%]">{account.name}</p>
                <p>{formatMoney(account.balance)}</p>
            </li>
        {/each}
        <Button variant="neutral" size="small">
            <!--TODO: change stoke width-->
            <Plus size="12"/>
            Add account
        </Button>
    </ul>
    <!---TODO: check border-->
    <!---TODO: change icon-->
    <div class="nav-panel-actions">
        <SettingsButton/>
        <IconButton
            variant="ghost"
            aria-expanded={expanded}
            aria-controls="nav-panel"
            onclick={() => expanded = !expanded}
        >
            <PanelRightOpen/>
        </IconButton>
    </div>
</aside>

<style>
	aside{
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
        gap: 24px;
        padding-top: 12px;
		background-color: var(--color-neutral-25);
        border-right: 1px solid var(--color-neutral-100);

        &:global([data-expanded="false"]){
            width: fit-content;
        }
	}

    .account-list{
        display: flex;
        flex-direction: column;
        gap: 12px;
        padding-inline: 20px;

        :global([data-expanded="false"]) &{
            display: none;
        }
    }

    li{
        display: flex;
        justify-content: space-between;
    }

    .page-link{
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 8px 16px;

        :global(svg){
            width: 16px;
            height: 16px;
        }

        p{
            :global([data-expanded="false"]) &{
                display: none;
            }
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
