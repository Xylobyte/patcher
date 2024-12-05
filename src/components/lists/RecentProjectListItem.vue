<script lang="ts" setup>
import {convertISOToLocalDate} from "../../utils/dates.ts"
import SmallButton from "../buttons/SmallButton.vue"
import {Project} from "../../interfaces/configs.ts"
import {EllipsisVertical, FolderSearch} from "lucide-vue-next"
import {message} from "@tauri-apps/plugin-dialog"
import {open} from '@tauri-apps/plugin-shell'

const props = defineProps<{
    project: Project
    homePath: string
}>()

defineEmits<{
    openProject: [path: string],
    showContextMenu: [e: Event, context: Project]
}>()

const pathError = (p: string) => {
    message(`The path ${p} doesn't exist!`, {title: `Path not exists`, type: "error"})
}
</script>

<template>
    <li
        class="flex gap15 align-center space-between border-r-small"
        @click="props.project.path_exists ? $emit('openProject', props.project.path) : pathError(props.project.path)"
    >
        <div class="flex column gap5">
            <div class="head flex gap10 align-center">
                <h3 class="ellipsis">{{ props.project.name }}</h3>-
                <span>{{ convertISOToLocalDate(props.project.last_opened) }}</span>
            </div>
            <span :class="[props.project.path_exists ? 'valid' : 'invalid']" class="path ellipsis">
                {{props.project.path.startsWith(props.homePath) ? '~' + props.project.path.substring(props.homePath.length - 1) : props.project.path }}
            </span>
        </div>

        <div class="flex gap5">
            <SmallButton
                @click.stop="props.project.path_exists ? open(props.project.path) : pathError(props.project.path)">
                <FolderSearch/>
            </SmallButton>

            <SmallButton @click.stop="e => $emit('showContextMenu', e, props.project)">
                <EllipsisVertical/>
            </SmallButton>
        </div>
    </li>
</template>

<style lang="scss" scoped>
li {
    background: var(--color-background-light);
    padding: 8px;

    &:hover {
        background: var(--color-background-scroll);
    }

    .head {
        width: 100%;
        height: 30px;

        h3 {
            font-weight: bold;
            max-width: calc(50vw / 2);
        }

        span {
            color: var(--color-font-light);
            font-size: 0.85em;
            font-weight: 300;
        }
    }

    .path {
        color: var(--color-font-light);
        font-size: 0.75em;
        font-weight: 300;
        max-width: calc(50vw / 1.3);

        &.invalid {
            color: var(--color-error);
        }
    }
}
</style>