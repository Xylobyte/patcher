<script lang="ts" setup>
import {computed, onMounted, onUnmounted, ref} from "vue"
import {Project} from "../interfaces/configs.ts"
import {homeDir} from "@tauri-apps/api/path"
import {invoke} from "@tauri-apps/api/tauri"
import {message, open as pickFolder} from "@tauri-apps/api/dialog"
import {writeText} from "@tauri-apps/api/clipboard"
import {emit} from "@tauri-apps/api/event"
import {SEND_NOTIFICATION} from "../utils/data/events-names.ts"
import {AddNotification} from "../interfaces/notifications.ts"
import BaseButton from "../components/buttons/BaseButton.vue"
import CustomScrollbar from "../components/CustomScrollbar.vue"
import {convertISOToLocalDate} from "../utils/dates.ts"
import SmallButton from "../components/buttons/SmallButton.vue"
import {EllipsisVertical, FolderSearch} from "lucide-vue-next"
import {open} from '@tauri-apps/api/shell'
import {contextMenuItems} from "../utils/data/context-menu-items.ts"
import {ContextMenuItem} from "../interfaces/context-menu.ts"
import ContextMenu from "../components/ContextMenu.vue"

const projects = ref<Project[]>([])
const sortedProjects = computed(() => projects.value.sort((a, b) =>
	new Date(b.last_opened).getTime() - new Date(a.last_opened).getTime()
))

const homePath = ref<string>("null")
const contextMenu = ref<{
	x: number
	y: number
	items: ContextMenuItem[]
	context: Project
} | null>(null)

const refreshProjects = async () => {
	try {
		projects.value = await invoke<Project[]>('get_recent_projects')
	} catch (error) {
		console.error(error)
	}
}

onMounted(async () => {
	homePath.value = await homeDir()
	await refreshProjects()

	window.addEventListener('focus', refreshProjects)
})
onUnmounted(() => {
	window.removeEventListener('focus', refreshProjects)
})

const pathError = (p: string) => {
	message(`The path ${p} doesn't exist!`, {title: `Path not exists`, type: "error"})
}

const execAction = async (action: string) => {
	if (contextMenu.value?.context) {
		switch (action) {
			case "copyPath":
				await writeText(contextMenu.value?.context.path)
                emit(SEND_NOTIFICATION, {
					message: "Le chemin a bien été copié",
					type: "info"
				} as AddNotification)
				break
			case "removeFromList":
				projects.value = await invoke<Project[]>('remove_project', {path: contextMenu.value?.context.path})
				emit(SEND_NOTIFICATION, {
					message: "Le projet a bien été retiré de la liste",
					type: "info"
				} as AddNotification)
				break
		}
	}

    contextMenu.value = null
}

const openFolder = async () => {
	const select = await pickFolder({
		directory: true,
		multiple: false,
		title: "Choisir un dossier de projet",
		defaultPath: homePath.value
	})

	await openProject(select as string)
}

const openProject = async (p: string) => {
}

const showContextMenu = (e: Event, context: Project) => {
	const elBounding = (e.currentTarget as HTMLElement).getBoundingClientRect()
	contextMenu.value = {
		x: elBounding.left + elBounding.width / 2,
		y: elBounding.top + elBounding.height / 2,
		items: contextMenuItems,
		context: context
	}
}
</script>

<template>
	<main id="recent-projects" class="flex column align-center justify-center gap30">
		<h1>Liste de vos APIs</h1>

		<div class="flex align-center gap10">
			<BaseButton :is-primary="true" @click="openFolder">
				Ouvrir un projet
			</BaseButton>
			<BaseButton>
				Créer un projet vide
			</BaseButton>
		</div>

		<section class="grid-center">
			<CustomScrollbar v-if="sortedProjects.length > 0">
				<ul class="p-list flex column gap10">
					<li
						v-for="p in sortedProjects"
						:key="p.path"
						class="flex gap15 align-center space-between border-r-small"
						@click="() => p.path_exists ? openProject(p.path) : pathError(p.path)"
					>
						<div class="flex column gap5">
							<div class="head flex gap10 align-center">
								<h3 class="ellipsis">{{ p.name }}</h3>-
								<span>{{ convertISOToLocalDate(p.last_opened) }}</span>
							</div>
							<span :class="[p.path_exists ? 'valid' : 'invalid']" class="path ellipsis">
                                {{ p.path.startsWith(homePath) ? '~' + p.path.substring(homePath.length - 1) : p.path }}
                            </span>
						</div>

						<div class="flex gap5">
							<SmallButton @click.stop="p.path_exists ? open(p.path) : pathError(p.path)">
								<FolderSearch/>
							</SmallButton>

							<SmallButton @click.stop="e => showContextMenu(e, p)">
								<EllipsisVertical/>
							</SmallButton>
						</div>
					</li>
				</ul>
			</CustomScrollbar>
			<span v-else class="no-projects">
                Aucun projets récents
            </span>
		</section>

		<ContextMenu
			v-if="contextMenu"
			:items="contextMenu.items"
			:x="contextMenu.x"
			:y="contextMenu.y"
            @action="execAction"
			@close="contextMenu = null"
		/>
	</main>
</template>

<style lang="scss" scoped>
#recent-projects {
	section {
		width: 50vw;
		min-width: 550px;
		height: 50vh;
		position: relative;
	}

	.p-list li {
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

	.no-projects {
		color: var(--color-font-light);
		font-size: 1.1em;
		font-weight: normal;
		text-align: center;
		width: 100%;
	}
}
</style>