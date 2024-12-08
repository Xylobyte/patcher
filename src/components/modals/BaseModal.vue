<script lang="ts" setup>
import {onMounted, onUnmounted} from "vue"
import BaseButton from "../buttons/BaseButton.vue"

defineProps<{
    title: string
}>()

const emit = defineEmits<{
    close: [],
    confirm: []
}>()

onMounted(() => {
    document.addEventListener('keydown', handleKeydown)
})
onUnmounted(() => {
    document.removeEventListener('keydown', handleKeydown)
})

const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === "Escape") emit('close')
}
</script>

<template>
    <Teleport to="body">
        <div class="modal-wrapper grid-center gap20" @mousedown.self="$emit('close')">
            <div class="modal flex column gap20 border-r">
                <h2>{{ $props.title }}</h2>

                <slot/>

                <div class="actions flex gap10">
                    <slot name="actions">
                        <BaseButton @click="$emit('close')">Fermer</BaseButton>
                        <BaseButton :is-primary="true" @click="$emit('confirm')">Confirmer</BaseButton>
                    </slot>
                </div>
            </div>
        </div>
    </Teleport>
</template>

<style lang="scss" scoped>
.modal-wrapper {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 20;
}

.modal {
    padding: 20px;
    max-height: 90vh;
    max-width: 90vw;
    background: var(--color-background);

    .actions {
        justify-content: flex-end;
    }
}
</style>