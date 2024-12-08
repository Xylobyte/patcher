<script lang="ts" setup>
import LineSeparator from "../LineSeparator.vue";
import PathSelectInput from "../inputs/PathSelectInput.vue";
import SingleLineInput from "../inputs/SingleLineInput.vue";
import {OpenAPIObject} from "../../interfaces/oas3.ts";
import {ref} from "vue";
import BaseModal from "./BaseModal.vue";

const {modalInitProject} = defineProps<{
    modalInitProject: { path: string, projectInfo: OpenAPIObject }
}>();

defineEmits<{
    close: [],
    confirm: [title: string, desc: string, serverUrl: string, path: string]
}>()

const title = ref(modalInitProject.projectInfo.info.title);
const desc = ref(modalInitProject.projectInfo.info.description || "");
const serverUrl = ref(modalInitProject.projectInfo.servers?.[0].url || "");
const path = ref(modalInitProject.path);
</script>

<template>
    <BaseModal
        title="Nouveau projet"
        @close="$emit('close')"
        @confirm="$emit('confirm', title, desc, serverUrl, path)"
    >
        <div class="flex gap10">
            <div class="flex column gap15">
                <SingleLineInput
                    v-model="title"
                    :auto-focus="true"
                    label="Nom du projet"
                />
                <SingleLineInput
                    v-model="desc"
                    :auto-focus="true"
                    label="Description du projet"
                />
            </div>

            <LineSeparator orientation="vertical"/>

            <div class="flex column gap15">
                <SingleLineInput
                    v-model="serverUrl"
                    :auto-focus="true"
                    class="large-input"
                    label="URL root de l'API"
                    placeholder="ex: http://localhost:3000/api"
                />

                <PathSelectInput
                    v-model="path"
                    :is-folder="true"
                    :path-editable="true"
                    :show-path="true"
                    label="Dossier du projet"
                />
            </div>
        </div>
    </BaseModal>
</template>

<style lang="scss" scoped>
</style>