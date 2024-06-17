<script lang="ts" setup>
import {onMounted, onUnmounted, ref} from "vue";
import {AddNotification, Notification} from "../interfaces/notifications.ts";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {SEND_NOTIFICATION} from "../utils/events-names.ts";
import {CircleCheck, Info, OctagonX} from "lucide-vue-next";

const notifications = ref<Notification[]>([])

let unsubscribe: UnlistenFn;
onMounted(async () => {
    unsubscribe = await listen<AddNotification>(SEND_NOTIFICATION, (e) => {
        const id = Date.now();
        notifications.value.unshift({
            message: e.payload.message,
            id: id,
            type: e.payload.type
        });

        setTimeout(() => {
            notifications.value = notifications.value.filter(n => n.id !== id)
        }, 4000);
    })
})
onUnmounted(() => {
    unsubscribe();
})
</script>

<template>
    <div class="notifications-center flex column gap10 align-center">
        <div v-for="notification in notifications" :key="notification.id"
             class="notification b-shadow flex align-center gap10 border-r-small {{notification.type}}">
            <Info v-if="notification.type === 'info'" :size="16"/>
            <OctagonX v-else-if="notification.type === 'error'" :size="16"/>
            <CircleCheck v-else-if="notification.type === 'success'" :size="16"/>
            <span>{{ notification.message }}</span>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.notifications-center {
    position: fixed;
    top: 20px;
    left: 0;
    right: 0;
    z-index: 300;
    pointer-events: none;

    .notification {
        pointer-events: all;
        padding: 10px;
        font-size: 0.9em;
        animation: forwards popupFromTop 0.3s;

        &.info {
            background: var(--color-background-scroll);
            color: var(--color-font-light);
        }
    }

    @keyframes popupFromTop {
        from {
            transform: translateY(-100%);
        }
    }
}
</style>