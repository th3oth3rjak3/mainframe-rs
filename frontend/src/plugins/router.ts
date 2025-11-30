import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';

import DashboardPage from '@/views/DashboardPage.vue';
import LoginPage from '@/views/LoginPage.vue';
import { useUserStore } from '@/stores/user';

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'Dashboard',
        component: DashboardPage,
        meta: { requiresAuth: true },
    },
    {
        path: '/login',
        name: 'Login',
        component: LoginPage,
        meta: { public: true },
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

const router = createRouter({
    history: createWebHistory(),
    routes
});


router.beforeEach((to) => {
    const auth = useUserStore();          // get your auth state

    // Public routes require no check
    if (to.meta.public) return true;

    // If route requires auth but user isn’t logged in → login
    if (to.meta.requiresAuth && !auth.isLoggedIn) {
        return { name: 'Login' };
    }

    // Require admin → block to home/dashboard
    if (to.meta.requiresAdmin && !auth.currentUser?.isAdmin) {
        return { name: 'Dashboard' }; // or a 403 page
    }

    return true; // allow navigation
});


export default router;