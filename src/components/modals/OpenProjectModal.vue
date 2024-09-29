<script lang="ts" setup>
import LineSeparator from "../LineSeparator.vue";
import PathSelectInput from "../inputs/PathSelectInput.vue";
import SingleLineInput from "../inputs/SingleLineInput.vue";
import {ProjectInfo} from "../../interfaces/projects.ts";

const {modalInitProject} = defineProps<{
    modalInitProject: { path: string, projectInfo: ProjectInfo }
}>()
</script>

<template>
    <div class="flex gap10">
        <div class="flex column gap15">
            <SingleLineInput
                v-model="modalInitProject.projectInfo.name"
                :auto-focus="true"
                label="Nom du projet"
            />
            <SingleLineInput
                v-model="modalInitProject.projectInfo.description"
                :auto-focus="true"
                label="Description du projet"
            />
        </div>

        <LineSeparator orientation="vertical"/>

        <div class="flex column gap15">
            <SingleLineInput
                v-model="modalInitProject.projectInfo.root_url"
                :auto-focus="true"
                class="large-input"
                label="URL root de l'API"
                placeholder="ex: http://localhost:3000/api"
            />

            <PathSelectInput
                v-model="modalInitProject.path"
                :is-folder="true"
                :path-editable="true"
                :show-path="true"
                label="Dossier du projet"
            />
        </div>
    </div>
</template>

<style lang="scss" scoped>

</style>