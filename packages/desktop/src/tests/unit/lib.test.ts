import {getCurrencySymbol, parseMoney, parseDate, type Account} from "$lib/lib";
import { describe, expect, test } from "vitest";
import {AccountStore} from "$lib/account.svelte";
import {mockIPC} from "@tauri-apps/api/mocks";

describe("AccountStore",()=>{
    test("create a new account",async()=>{
        mockIPC((cmd,args)=>{
            if (cmd === "create_account"){
                const account: Account = {
                    id:"1",
                    name:args.name,
                    balance: "0.00",
                    startingBalance:args.startingBalance
                };
                return account;
            }
        });
        const accountStore = new AccountStore();
        const account = await accountStore.createAccount({name:"Account 10",startingBalance:"10.00"});
        expect(accountStore.accounts).toHaveLength(1);
        expect(accountStore.accounts[0].id).toBe(account.id);
        expect(accountStore.accounts[0].name).toBe("Account 10");
        expect(accountStore.accounts[0].startingBalance).toBe("10.00");
    });
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

describe("Parse date", () => {
    test("dd/MM/YYYY",() => {
        const date = parseDate("12/12/2025");
        expect(date).toStrictEqual(new Date(2025,11,12));
    });
});

