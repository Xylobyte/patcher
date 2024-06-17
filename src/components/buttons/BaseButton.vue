<script lang="ts" setup>
import {ButtonHTMLAttributes} from "vue"

defineProps<{
    isPrimary?: boolean
}>()
</script>

<template>
    <button :class="{isPrimary: isPrimary}" class="flex align-center border-r-small"
            v-bind="$attrs as ButtonHTMLAttributes">
        <slot/>
    </button>
</template>

<style lang="scss" scoped>
button {
    padding: 6px 12px;
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
