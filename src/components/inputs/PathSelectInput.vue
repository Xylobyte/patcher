<script lang="ts" setup>
defineProps<{
    label?: string
    isFolder?: boolean
    showPathInput?: boolean
}>()
</script>

<template>
    <div>
        <input type="file">
    </div>
</template>

<style lang="scss" scoped>

</style>