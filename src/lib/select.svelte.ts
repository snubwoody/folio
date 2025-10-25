// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
import { Select } from "melt/builders";

export type SelectOption = {
    /** Label for display purposes. */
    label: string;
    /** Unique value for identifying. */
    value: string;
};

// TODO: add default value
type UseSelectProps<T> = {
    items: T[];
    toOption: (item: T) => SelectOption;
    defaultValue?: T;
    onChange?: ({ item, option }: { item: T; option: SelectOption }) => void;
};

export function useSelect<T>(options: UseSelectProps<T>) {
    const { items, toOption, defaultValue, onChange } = options;

    const updateValue = (value?: SelectOption) => {
        const item = items.find((i) => toOption(i).value === value?.value);
        if (item && value) {
            onChange?.({ item: item, option: value });
        }
    };

    const opts = $derived(items.map((i) => toOption(i)));

    const select = new Select<SelectOption>({
        value: defaultValue ? toOption(defaultValue) : undefined,
        onValueChange: (value) => updateValue(value),
    });

    return { select, options: opts };
}
