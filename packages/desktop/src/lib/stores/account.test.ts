import { test,expect } from "vitest";
import { mockCreateAccount, accountBalance, accountStore } from "$lib/stores/account.svelte";
import { transactionStore } from "$lib/stores/transaction.svelte";

test("Calculate account balance",async () => {
    mockCreateAccount();
    const account = await accountStore.createTestAccount({ name:"" });
    transactionStore.addTestTransaction({
        amount: "500",
        fromAccountId: account.id
    });
    const balance = accountBalance(account.id,transactionStore.transactions);
    expect(balance).toEqual(-500);
});

test("Include starting balance in account balance",async () => {
    mockCreateAccount();
    const account = await accountStore.createTestAccount({ name:"",startingBalance:"1500.00" });

    transactionStore.addTestTransaction({
        fromAccountId: account.id,
        amount: "500"
    });
    const balance = accountBalance(account.id,transactionStore.transactions);
    expect(balance).toEqual(1000);
});