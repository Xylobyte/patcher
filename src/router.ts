import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";

const routes: RouteRecordRaw[] = [
    {path: '/', redirect: '/recent-projects'},
    {path: '/recent-projects', component: () => "Test"},
    {path: '/main', component: () => "Main"},
];

const router = createRouter({
    history: createWebHistory(),
    routes: routes
});

export default router;
