import { CsvLoader } from "$lib/utils/loader.svelte";
import {describe, expect, test} from "vitest";

// TODO: test different row lengths
describe("CsvLoader",() => {
    test("Load csv rows",() => {
        const records = [
            ["24/12/2026","Shoprite","24.0","0.0","65.0"],
            ["31/12/2026","Uber GA","10.0","0.0","25.0"],
        ];
        const loader = new CsvLoader(records);
    });

    test("Columns length matches row length",() => {
        const records = [
            ["24/12/2026","Shoprite","24.0","0.0","65.0"],
            ["31/12/2026","Uber GA","10.0","0.0","25.0"],
        ];
        const loader = new CsvLoader(records);
        expect(loader.columns).toHaveLength(loader.rows[0].length);
    });

    test("Delete row",() => {
        const records = [
            ["24/12/2026","Shoprite","24.0","0.0","65.0"],
            ["31/12/2026","Uber GA","10.0","0.0","25.0"],
            ["31/12/2026","Choppies SA","10.0","0.0","25.0"],
            ["31/12/2026","Debonairs SOL","10.0","0.0","25.0"],
        ];

        const loader = new CsvLoader(records);

        loader.deleteRow(2);

        expect(loader.rows).toHaveLength(3);
        expect(loader.rows[0][1]).toBe("Shoprite");
        expect(loader.rows[1][1]).toBe("Uber GA");
        expect(loader.rows[2][1]).toBe("Debonairs SOL");
    });

    test("Delete row index < 0",() => {
        const records = [
            ["24/12/2026","Shoprite","24.0","0.0","65.0"],
            ["31/12/2026","Uber GA","10.0","0.0","25.0"],
            ["31/12/2026","Choppies SA","10.0","0.0","25.0"],
            ["31/12/2026","Debonairs SOL","10.0","0.0","25.0"],
        ];

        const loader = new CsvLoader(records);

        loader.deleteRow(-1);
        expect(loader.rows).toHaveLength(4);
    });

    test("Delete row index > length",() => {
        const records = [
            ["24/12/2026","Shoprite","24.0","0.0","65.0"],
            ["31/12/2026","Uber GA","10.0","0.0","25.0"],
            ["31/12/2026","Choppies SA","10.0","0.0","25.0"],
            ["31/12/2026","Debonairs SOL","10.0","0.0","25.0"],
        ];

        const loader = new CsvLoader(records);

        loader.deleteRow(10);
        expect(loader.rows).toHaveLength(4);
    });

    test("Delete row index == length",() => {
        const records = [
            ["24/12/2026","Shoprite","24.0","0.0","65.0"],
            ["31/12/2026","Uber GA","10.0","0.0","25.0"],
            ["31/12/2026","Choppies SA","10.0","0.0","25.0"],
            ["31/12/2026","Debonairs SOL","10.0","0.0","25.0"],
        ];

        const loader = new CsvLoader(records);

        loader.deleteRow(4);
        expect(loader.rows).toHaveLength(4);
    });
});