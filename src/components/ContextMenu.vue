<script lang="ts" setup>
import {computed, CSSProperties, onMounted, onUnmounted, ref} from "vue"
import {ContextMenuItem} from "../interfaces/context-menu.ts"

interface ContextMenuProps {
    items: ContextMenuItem[]
    x: number
    y: number
}

const props = defineProps<ContextMenuProps>()
const emit = defineEmits<{
    close: []
    action: [action: string]
}>()

const menuRef = ref<HTMLButtonElement | null>(null)

const style = computed(() => ({
    top: props.y + "px",
    left: props.x + "px"
} as CSSProperties))

onMounted(() => {
    console.log(style.value)
    document.addEventListener('mousedown', handleClickOutside)
})
onUnmounted(() => {
    document.removeEventListener('mousedown', handleClickOutside)
})

const handleClickOutside = (event: MouseEvent) => {
    if (menuRef.value && !menuRef.value.contains(event.target as Node)) {
        emit('close')
    }
}
</script>

<template>
    <div
        ref="menuRef"
        :style="style"
        class="context-menu b-shadow border-r-small"
    >
        <ul class="flex column">
            <li v-for="item in items" @click="() => $emit('action', item.action)">
                {{ item.name }}
            </li>
        </ul>
    </div>
</template>

<style lang="scss" scoped>
.context-menu {
    position: absolute;
    background: var(--color-background-scroll);
    z-index: 200;
    overflow: hidden;

    li {
        padding: 10px;
        font-size: 0.8em;

        &:hover {
            background: var(--color-background-button);
        }
    }
}
</style>