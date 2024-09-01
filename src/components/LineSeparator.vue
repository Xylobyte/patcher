<script lang="ts" setup>
defineProps<{
    orientation: "horizontal" | "vertical"
}>()
</script>

<template>
    <hr :class="$props.orientation">
</template>

<style lang="scss" scoped>
hr {
    background: var(--color-font-light);
    flex: 1;

    &.horizontal {
        height: 1px;
    }

    &.vertical {
        width: 1px;
    }
}
</style>