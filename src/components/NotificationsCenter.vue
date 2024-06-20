<script lang="ts" setup>
import {onMounted, onUnmounted, ref} from "vue"
import {AddNotification, Notification} from "../interfaces/notifications.ts"
import {listen, UnlistenFn} from "@tauri-apps/api/event"
import {SEND_NOTIFICATION} from "../utils/data/events-names.ts"
import {CircleCheck, Info, OctagonX} from "lucide-vue-next"

const notifications = ref<Notification[]>([])

let unsubscribe: UnlistenFn
onMounted(async () => {
    console.log("Notification listen start")

    unsubscribe = await listen<AddNotification>(SEND_NOTIFICATION, (e) => {
        console.log("Notification")
        const id = Date.now()
        notifications.value.unshift({
            message: e.payload.message,
            id: id,
            type: e.payload.type
        })

        setTimeout(() => {
            notifications.value = notifications.value.filter(n => n.id !== id)
        }, 4000)
    })
})
onUnmounted(() => {
    unsubscribe()
})
</script>

<template>
    <div class="notifications-center flex column align-center">
        <TransitionGroup class="flex column gap10" name="notification" tag="div">
            <div
                v-for="notification in notifications"
                :key="notification.id"
                :class="[notification.type]"
                class="notification b-shadow flex align-center gap10 border-r-small"
            >
                <Info v-if="notification.type === 'info'" :size="16"/>
                <OctagonX v-else-if="notification.type === 'error'" :size="16"/>
                <CircleCheck v-else-if="notification.type === 'success'" :size="16"/>
                <span>{{ notification.message }}</span>
            </div>
        </TransitionGroup>
    </div>
</template>

<style lang="scss" scoped>
.notification-move,
.notification-enter-active,
.notification-leave-active {
    transition: all 0.4s ease;
}
.notification-enter-from {
    opacity: 0;
    transform: translateY(-100%);
}
.notification-leave-to {
    opacity: 0;
    transform: translateX(-30px);
}

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

        &.info {
            background: var(--color-background-scroll);
            color: var(--color-font-light);
        }
    }
}
</style>