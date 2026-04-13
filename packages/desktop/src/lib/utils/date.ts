import { CalendarDate, getLocalTimeZone } from "@internationalized/date";

export function formatDate(date: CalendarDate): string {
    return Intl.DateTimeFormat("en-US", { dateStyle: "medium" }).format(date.toDate(getLocalTimeZone()));
}

// The goal of this function to parse the input and always return a valid date.
export function parseDate(value: string, reference: CalendarDate): CalendarDate {
    // TODO: use backend command
    const singleDigit = /^\d+$/.test(value.trim());
    if (singleDigit) {
        return reference.set({ day: parseInt(value.trim()) });
    }
    // Test constraints
    console.log(singleDigit);

    // return previous date if all else fail.
    return reference;

}