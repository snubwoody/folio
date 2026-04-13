import {CalendarDate} from "@internationalized/date";

export function formatDate(dateStr: string): string {
    // TODO: use CalendarDate
    const [year, month, day]: string[] = dateStr.split("-");
    const date = new Date(Number(year), Number(month) - 1, Number(day));
    return Intl.DateTimeFormat("en-US", {dateStyle: "medium"}).format(date);
}

// The goal of this function to parse the input and always return a valid date.
export function parseDate(value: string, reference: CalendarDate): CalendarDate {
    // TODO: use backend command
    const singleDigit = /^\d+$/.test(value.trim());
    if (singleDigit) {
        return reference.set({day: parseInt(value.trim())});
    }
    // Test constraints
    console.log(singleDigit);

    // return previous date if all else fail.
    return reference;

}