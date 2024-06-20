import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router"
import RecentProjectsPage from "./pages/RecentProjectsPage.vue"

const routes: RouteRecordRaw[] = [
    {path: '/', redirect: '/recent-projects'},
    {path: '/recent-projects', component: RecentProjectsPage},
    {path: '/main', component: RecentProjectsPage}
]

const router = createRouter({
    history: createWebHistory(),
    routes: routes
})

export default router
