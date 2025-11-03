import { getCurrencySymbol, parseMoney } from "$lib/lib";
import { expect, test } from "vitest";

test("Get currency symbol",() => {
    expect(getCurrencySymbol("USD")).toBe("$");
    expect(getCurrencySymbol("ZAR")).toBe("ZAR");
});

test("Format money",() => {
    const money = parseMoney("224");
    expect(money).toBe("224");
});

test("Format money with decimal",() => {
    const money = parseMoney("224.2442");
    expect(money).toBe("224.2442");
});

test("Format money with comma",() => {
    const money = parseMoney("2,2424.24");
    expect(money).toBe("22424.24");
});

test("Format money with multiple commas",() => {
    const money = parseMoney("2,24,24.24");
    expect(money).toBe("22424.24");
});

test("Format money with multiple decimal points",() => {
    const money = parseMoney("2.24.24.24");
    expect(money).toBe("2.24");
});

test("Format money with characters",() => {
    const money = parseMoney("2,24wr.24");
    expect(money).toBe("224");
});

test("Format random characters",() => {
    const money = parseMoney("wkkrwr");
    expect(money).not.toBeDefined();
});