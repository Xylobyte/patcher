<script lang="ts" setup>
import {Check, X} from "lucide-vue-next"

defineProps<{
    label?: string
}>()

const checkModel = defineModel<boolean>()
</script>

<template>
    <div class="flex align-center gap10">
        <span v-if="$props.label" class="label">{{ $props.label }}</span>

        <label :class="{checked: checkModel}" class="switch border">
            <input v-model="checkModel" type="checkbox">
            <span class="grid-center border-r">
                <Check v-if="checkModel" :size="13"/>
                <X v-else :size="13" color="black"/>
            </span>
        </label>
    </div>
</template>

<style lang="scss" scoped>
.label {
    color: var(--color-font-light);
}

input {
    opacity: 0;
    width: 0;
    height: 0;
}

.switch {
    position: relative;
    display: inline-block;
    width: 45px;
    height: 25px;
    border-radius: 15px;
    cursor: pointer;
    transition: 0.3s;

    span {
        position: absolute;
        top: 50%;
        left: 0;
        width: 18px;
        height: 18px;
        transform: translateX(calc(7px / 2)) translateY(-50%);
        background-color: var(--color-primary);
        transition: 0.3s;
        transition-property: background-color, transform;
    }

    &.checked {
        border-color: var(--color-primary);
        background-color: var(--color-primary);

        span {
            background-color: var(--color-background) !important;
            transform: translateX(100%) translateY(-50%);
            width: 20px;
            height: 20px;
        }
    }
}
</style>