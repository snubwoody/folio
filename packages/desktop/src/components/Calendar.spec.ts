import { expect, test,describe } from "vitest";
import { render } from "vitest-browser-svelte";
import Calendar from "./Calendar.svelte";
import { CalendarDate, getLocalTimeZone, today, type DateValue } from "@internationalized/date";

describe("Calendar",() => {
    const unused = 0;
    test("today button sets current date",async() => {
        let date: DateValue | undefined;
        const screen = render(Calendar,{ onDateChange: value => date = value, value: new CalendarDate(1,1,1) });
        expect(date).toBeUndefined();
        await screen.getByRole("button",{ name: "Today" }).click();
        expect(date).toStrictEqual(today(getLocalTimeZone()));
    });
    test("next month button increments month",async() => {
        const screen = render(Calendar,{ value: new CalendarDate(2020,1,1) });
        await expect.element(screen.getByRole("heading")).toHaveTextContent("January 2020");
        await screen.getByRole("button",{ name: "Next" }).click();
        await expect.element(screen.getByRole("heading")).toHaveTextContent("February 2020");
    });
    test("previous month button decrements month",async() => {
        const screen = render(Calendar,{ value: new CalendarDate(2020,10,1) });
        await expect.element(screen.getByRole("heading")).toHaveTextContent("October 2020");
        await screen.getByRole("button",{ name: "Previous" }).click();
        await expect.element(screen.getByRole("heading")).toHaveTextContent("September 2020");
    });
});