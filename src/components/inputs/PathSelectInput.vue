<script lang="ts" setup>
import BaseButton from "../buttons/BaseButton.vue";
import {open as pickFolder} from "@tauri-apps/plugin-dialog"

const props = defineProps<{
    label?: string
    defaultPath?: string
    pickerTitle?: string
    isFolder?: boolean
    showPath?: boolean
    pathEditable?: boolean
}>()

const pathModel = defineModel<string>()

const pickPath = async () => {
    pathModel.value = await pickFolder({
        directory: props.isFolder,
        multiple: false,
        title: props.pickerTitle || "Choisir un dossier de projet",
        defaultPath: props.defaultPath || pathModel.value
    }) as string || pathModel.value
}
</script>

<template>
    <div class="flex column gap5">
        <label v-if="$props.label">{{ $props.label }}</label>
        <div :class="{border: $props.showPath}" class="picker flex gap10 border-r-small">
            <BaseButton is-primary @click="pickPath">
                {{ `Choisir un ${$props.isFolder ? 'dossier' : 'fichier'}` }}
            </BaseButton>
            <input
                v-show="$props.showPath"
                v-model="pathModel"
                :readonly="!$props.pathEditable"
                type="text"
            >
        </div>
    </div>
</template>

<style lang="scss" scoped>
.picker {
    width: 100%;
    color: var(--color-font);
}

.border {
    padding: 4px;
}

:deep(button) {
    font-size: 0.75em;
    padding: 6px 8px;
    min-width: max-content;
}

input {
    background: transparent;
    width: 100%;
}
</style>