import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';

import DashboardPage from '@/views/DashboardPage.vue';
import LoginPage from '@/views/LoginPage.vue';

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'Dashboard',
        component: DashboardPage,
    },
    {
        path: '/login',
        name: 'Login',
        component: LoginPage,
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