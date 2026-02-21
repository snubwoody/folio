import { describe,test,expect } from "vitest";
import { parseDate } from "@internationalized/date";
import { console } from "inspector";

describe("parseDate",()=>{
    test("parses dd/MM/yyyy",()=>{
        const date = parseDate("31/12/2026");
        console.log(date);
    });
});