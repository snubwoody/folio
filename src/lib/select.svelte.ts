import { Combobox,Select } from "melt/builders";

export type SelectOption = {
    /** Label for display purposes. */
    label: string,
    /** Unique value for identifying. */
    value: string
}

// TODO: add default value
type UseSelectProps<T> = {
    items: T[],
    toOption: (item: T) => SelectOption,
    defaultValue?: T,
    onChange?: ({ item, option }: {item: T,option: SelectOption}) => void
}


export function useSelect<T>(options: UseSelectProps<T>){
    const { items,toOption,defaultValue,onChange } = options;

    // TODO: test this;
    const  updateValue = (value?: SelectOption) => {
        const item = items.find(i => toOption(i).value === value?.value);
        if(item && value){
            onChange?.({ item:item,option:value });
        }
    };

	const opts = items.map(i => toOption(i));


    const select = new Select<SelectOption>({
        value: defaultValue ? toOption(defaultValue):undefined,
        onValueChange: (value) => updateValue(value),
    });

    return {select,options: opts};
}

