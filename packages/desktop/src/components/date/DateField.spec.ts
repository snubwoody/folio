import { expect, test, beforeEach } from "vitest";
import { render } from "vitest-browser-svelte";
import {parseDate, toCalendarDate} from "@internationalized/date";
import DateField from "./DateField.svelte";
import { formatDate } from "$lib/utils/date";
import { userEvent } from "vitest/browser";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";

beforeEach(() => {
    clearMocks();
});

test("Format date", async () => {
    const date = parseDate("2020-01-01");
    const screen = await render(DateField,{ value: date });
    await expect.element(screen.getByRole("textbox")).toHaveValue(formatDate(date));
});

test("Edit date on blur", async () => {
    mockIPC((cmd,args) => {
        if (cmd==="parse_date"){
            const payload = args as {value:string};
            expect(payload.value).toBe("Apr 1");
            return "2023-12-12";
        }
    });
    let date = parseDate("2020-01-01");
    const screen = await render(DateField,{ value: date,onDateChange: (d) => date = toCalendarDate(d) });
    const input = screen.getByRole("textbox");
    await expect.element(input).toHaveValue(formatDate(date));
    await input.fill("Apr 1");
    await userEvent.tab();
    await expect.element(input).toHaveValue(formatDate(parseDate("2023-12-12")));
    expect(date).toStrictEqual(parseDate("2023-12-12"));
});

test("Edit date after pressing enter", async () => {
    mockIPC((cmd,args) => {
        if (cmd==="parse_date"){
            const payload = args as {value:string};
            expect(payload.value).toBe("Apr 1");
            return "2023-12-12";
        }
    });
    let date = parseDate("2020-01-01");
    const screen = await render(DateField,{ value: date,onDateChange: (d) => date = toCalendarDate(d) });
    const input = screen.getByRole("textbox");
    await expect.element(input).toHaveValue(formatDate(date));
    await input.fill("Apr 1");
    await userEvent.keyboard("{Enter}");
    await expect.element(input).toHaveValue(formatDate(parseDate("2023-12-12")));
    expect(date).toStrictEqual(parseDate("2023-12-12"));
});