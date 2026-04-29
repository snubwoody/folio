import {test,expect} from "vitest";
import type {Transaction} from "$lib/api/transaction";
import {CalendarDate} from "@internationalized/date";
import {mockCreateAccount, accountBalance, accountStore} from "$lib/stores/account.svelte";
import {transactionStore} from "$lib/stores/transaction.svelte";

test("Calculate account balance",async ()=>{
    mockCreateAccount();
    const account = await accountStore.createTestAccount({name:"",});

    const expense: Transaction = {
        id: "1",
        fromAccountId: account.id,
        amount: "500.00",
        date: new CalendarDate(2025,1,1)
    };
    transactionStore.addTestTransaction(expense);
    const balance = accountBalance(account.id,transactionStore.transactions);
    expect(balance).toEqual(-500);
});

test("Include starting balance in account balance",async ()=>{
    mockCreateAccount();
    const account = await accountStore.createTestAccount({name:"",startingBalance:"1500.00"});

    const expense: Transaction = {
        id: "1",
        fromAccountId: account.id,
        amount: "500.00",
        date: new CalendarDate(2025,1,1)
    };
    transactionStore.addTestTransaction(expense);
    const balance = accountBalance(account.id,transactionStore.transactions);
    expect(balance).toEqual(1000);
});