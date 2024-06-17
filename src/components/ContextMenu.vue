<script lang="ts" setup>
import {computed, CSSProperties, onMounted, onUnmounted, ref} from "vue"

interface ContextMenuItem {
    name: string
    action: string
}

interface ContextMenuProps {
    items: ContextMenuItem[];
    x: number
    y: number
    onAction: (action: string) => void
    onClose: () => void
}

const props = defineProps<ContextMenuProps>()
const menuRef = ref<HTMLButtonElement | null>(null)

const style = computed(() => ({
    top: props.y,
    left: props.x
} as CSSProperties));

onMounted(() => {
    document.addEventListener('mousedown', handleClickOutside);
})
onUnmounted(() => {
    document.removeEventListener('mousedown', handleClickOutside);
})

const handleClickOutside = (event: MouseEvent) => {
    if (menuRef.value && !menuRef.value.contains(event.target as Node)) {
        props.onClose()
    }
}
</script>

<template>
    <div ref="menuRef" :style="style" class="context-menu b-shadow border-r-small">
        <ul class="flex column">
            <li v-for="item in items" @click="() => onAction(item.action)">
                {{ item.action }}
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