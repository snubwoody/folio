import { CalendarDate, getLocalTimeZone,parseDate as parseCalendarDate } from "@internationalized/date";
import { invoke } from "@tauri-apps/api/core";

export function formatDate(date: CalendarDate): string {
    return Intl
        .DateTimeFormat("en-GB", { dateStyle: "medium" })
        .format(date.toDate(getLocalTimeZone()));
}

export async function parseDate(value: string): Promise<CalendarDate> {
    const date = await invoke<string>("parse_date",{ value });
    return parseCalendarDate(date);
}