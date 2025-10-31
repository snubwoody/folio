<script lang="ts">
    interface Props {
        onChange?: (value:string) => void,
        value?: string,
        class?: string,
    };

    const { onChange, value, class: userClass }: Props = $props();

    let inner = $state(value ?? "");

    const update = () => {
        onChange?.(inner);
    };
</script>

<label class="inline-text-field">
    <input
        type="text"
        bind:value={inner}
        onblur={update}
        class={`${userClass}`}
    />
</label>

<style>
    .inline-text-field {
        position: relative;

        &::after {
            position: absolute;
            content: "";
            height: 1px;
            bottom: -4px;
            left: 0;
            width: 100%;
            background-color: transparent;
            transition: all 250ms;
        }

        &:focus-within {
            &::after {
                background-color: black;
            }
        }

        input {
            outline: none;
        }
    }
</style>
