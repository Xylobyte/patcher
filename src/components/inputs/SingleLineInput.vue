<script lang="ts" setup>
defineProps<{
    placeholder?: string
    label?: string
    class?: string
    autoFocus?: boolean
}>()

const inputModel = defineModel<string>()
</script>

<template>
    <div class="flex column gap5">
        <label v-if="$props.label">{{ $props.label }}</label>
        <input
            v-model="inputModel"
            :autofocus="$props.autoFocus"
            :class="$props.class"
            :placeholder="placeholder"
            class="border-r-small"
        />
    </div>
</template>

<style lang="scss" scoped>
label {
    font-size: 0.8em;
    font-weight: 300;
    color: var(--color-font-light);
}

input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid var(--color-background-button);
    color: var(--color-font);
    background: none;

    &:focus {
        outline: none;
        border: 1px solid var(--color-primary);
    }
}
</style>