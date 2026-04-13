import { type Account } from "$lib/lib";
import { describe, expect, test } from "vitest";
import { accountBalance, AccountStore } from "$lib/stores/account.svelte";
import { mockIPC } from "@tauri-apps/api/mocks";
import type { Transaction } from "./transaction";
import { CalendarDate, getLocalTimeZone, today } from "@internationalized/date";
import {getCurrencySymbol, parseMoney} from "$lib/utils/money";
import {parseDate} from "$lib/utils/date";

describe("parseDate",() => {
    test("parses single number as day of month",() => {
        expect(parseDate("10",new CalendarDate(2024,1,2))).toStrictEqual(new CalendarDate(2024,1,10));
        expect(parseDate("30",new CalendarDate(2024,2,1))).toStrictEqual(new CalendarDate(2024,2,29));
    });
});

describe("AccountStore",() => {
    test("create a new account",async() => {
        mockIPC((cmd,args) => {
            if (cmd === "create_account"){
                const payload = args as {name: string,startingBalance: string};
                const account: Account = {
                    id:"1",
                    name: payload.name,
                    balance: "0.00",
                    startingBalance:payload.startingBalance
                };
                return account;
            }
        });
        const accountStore = new AccountStore();
        const account = await accountStore.createAccount({ name:"Account 10",startingBalance:"10.00" });
        expect(accountStore.accounts).toHaveLength(1);
        expect(accountStore.accounts[0].id).toBe(account.id);
        expect(accountStore.accounts[0].name).toBe("Account 10");
        expect(accountStore.accounts[0].startingBalance).toBe("10.00");
    });
});

test("Account balance",() => {
    const expense: Transaction = {
        id:"",
        fromAccountId: "A1",
        amount: "5.00",
        date: today(getLocalTimeZone())
    };
    const income: Transaction = {
        id:"",
        toAccountId: "A1",
        amount: "25.00",
        date: today(getLocalTimeZone())
    };

    const balance = accountBalance("A1",[expense,income]);
    expect(balance).toBe(20);
});

test("Get currency symbol",() => {
    expect(getCurrencySymbol("USD")).toBe("$");
    expect(getCurrencySymbol("ZAR")).toBe("ZAR");
});

describe("Format money", () => {
    test("plain number",() => {
        const money = parseMoney("224");
        expect(money).toBe("224");
    });

    test("with decimal",() => {
        const money = parseMoney("224.2442");
        expect(money).toBe("224.2442");
    });

    test("with comma",() => {
        const money = parseMoney("2,2424.24");
        expect(money).toBe("22424.24");
    });

    test("multiple commas",() => {
        const money = parseMoney("2,24,24.24");
        expect(money).toBe("22424.24");
    });

    test("multiple decimal points",() => {
        const money = parseMoney("2.24.24.24");
        expect(money).toBe("2.24");
    });

    test("with characters",() => {
        const money = parseMoney("2,24wr.24");
        expect(money).toBe("224");
    });

    test("random characters",() => {
        const money = parseMoney("wkkrwr");
        expect(money).not.toBeDefined();
    });

});
