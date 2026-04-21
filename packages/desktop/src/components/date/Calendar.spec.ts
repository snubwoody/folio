import { expect, test } from "vitest";
import { render } from "vitest-browser-svelte";
import Calendar from "./Calendar.svelte";
import { CalendarDate, getLocalTimeZone, today, type DateValue } from "@internationalized/date";
import { mockIPC } from "@tauri-apps/api/mocks";
import { formatDate } from "$lib/utils/date";
import { userEvent } from "vitest/browser";

mockIPC((cmd) => {
    if (cmd === "parse_date"){
        return "2020-01-01";
    }
});

test("Has date field",async() => {
    const screen = await render(Calendar);
    await expect.element(screen.getByRole("textbox")).toBeInTheDocument();
});

// TODO: test calendar value changes with date field
test("Date field inherits value",async() => {
    const date =  new CalendarDate(2022,12,2);
    const screen = await render(Calendar,{ value:date });
    await expect.element(screen.getByRole("textbox")).toHaveValue(formatDate(date));
});

test("Value updates with date field",async() => {
    mockIPC((cmd) => {
        if (cmd === "parse_date"){
            return "2025-01-01";
        }
    });
    const date =  new CalendarDate(2022,12,2);
    const screen = await render(Calendar,{ value:date });
    const input = screen.getByRole("textbox");
    await input.clear();
    await input.click();
    await userEvent.keyboard("01/01/2025{Enter}");
    await expect.element(screen.getByRole("heading")).toHaveTextContent("January 2025");
});

test("Today button sets current date",async() => {
    let date: DateValue | undefined;
    const screen = await render(Calendar,{ onDateChange: value => date = value, value: new CalendarDate(1,1,1) });
    expect(date).toBeUndefined();
    await screen.getByRole("button",{ name: "Today" }).click();
    expect(date).toStrictEqual(today(getLocalTimeZone()));
});

test("Next month button increments month",async() => {
    const screen = await render(Calendar,{ value: new CalendarDate(2020,1,1) });
    await expect.element(screen.getByRole("heading")).toHaveTextContent("January 2020");
    await screen.getByRole("button",{ name: "Next" }).click();
    await expect.element(screen.getByRole("heading")).toHaveTextContent("February 2020");
});

test("Previous month button decrements month",async() => {
    const screen = await render(Calendar,{ value: new CalendarDate(2020,10,1) });
    await expect.element(screen.getByRole("heading")).toHaveTextContent("October 2020");
    await screen.getByRole("button",{ name: "Previous" }).click();
    await expect.element(screen.getByRole("heading")).toHaveTextContent("September 2020");
});
