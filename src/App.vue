<script lang="ts" setup>
import NotificationsCenter from "./components/NotificationsCenter.vue";
import DragHeader from "./components/DragHeader.vue";
</script>

<template>
    <DragHeader/>
    <NotificationsCenter/>

    <RouterView/>
</template>
