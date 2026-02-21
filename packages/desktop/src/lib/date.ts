import { format, parse } from "date-fns";


export function parseDate(str: string): Date{
    // TODO: handle failures
    // TODO:
    // - dd/MM/yyyy
    // - MM/dd/yyyy
    // - yyyy/MM/dd
    // - yyyy-MM-dd
    // - dd-MM-yyyy
    // - dd,MM yyyy
    // - MM,dd yyyy
    const date = parse(str,"dd/MM/yyyy", new Date());
    return date;
}