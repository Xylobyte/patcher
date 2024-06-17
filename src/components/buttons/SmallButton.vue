<template>
    <button class="grid-center border-r-small" v-bind="$attrs">
        <slot/>
    </button>
</template>

<style lang="scss" scoped>
button {
    background: transparent;
    padding: 5px;

    &:hover {
        background: var(--color-primary-light);
    }
}
</style>
