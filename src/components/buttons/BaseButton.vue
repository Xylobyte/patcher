<script lang="ts" setup>
defineProps<{
    isPrimary?: boolean
}>()
</script>

<template>
    <button
        :class="{primary: isPrimary}"
        class="flex align-center border-r-small"
        v-bind="$attrs">
        <slot/>
    </button>
</template>

<style lang="scss" scoped>
button {
    padding: 8px 12px;
    color: var(--color-font);
    background: var(--color-background-button);

    &.primary {
        background: var(--color-primary);
        color: var(--color-font-reverse);

        &:hover {
            background: var(--color-primary-light);
        }
    }

    &:hover {
        background: var(--color-background-scroll);
    }
}
</style>
