import { beforeEach, describe, expect, test } from "vitest";
import { render } from "vitest-browser-svelte";
import { accountStore } from "$lib/stores/account.svelte";
import { formatMoney } from "$lib/utils/money";
import AccountInfo from "./AccountInfo.svelte";

beforeEach(() => {
    accountStore.clear();
});

describe("AccountInfo", () => {
    test("displays account info", async () => {
        const account = await accountStore.createTestAccount({
            name: "My account",
            startingBalance: "500.00",
        });
        const screen = await render(AccountInfo, { accountId: account.id });

        expect(screen.getByText("My account")).toBeVisible();
        expect(screen.getByText(formatMoney("500.00"))).toBeVisible();
    });
});
