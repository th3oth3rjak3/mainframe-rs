import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';

import Dashboard from '@/views/Dashboard.vue';

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'Dashboard',
        component: Dashboard,
    },
    //   {
    //     path: '/recipes',
    //     name: 'Recipes', 
    //     component: Recipes
    //   },
    //   {
    //     path: '/recipes/:id',
    //     name: 'RecipeDetail',
    //     component: RecipeDetail,
    //     props: true  // Pass route params as props
    //   },
    //   {
    //     path: '/login',
    //     name: 'Login',
    //     component: Login
    //   },
    //   // Lazy load example
    //   {
    //     path: '/profile',
    //     name: 'Profile',
    //     component: () => import('@/views/Profile.vue') // lazy load
    //   }
]

export default createRouter({
    history: createWebHistory(),
    routes
});