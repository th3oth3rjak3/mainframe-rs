import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';

import DashboardPage from '@/views/DashboardPage.vue';
import LoginPage from '@/views/LoginPage.vue';
import { useUserStore } from '@/stores/user';
import RecipesListPage from '@/views/RecipesListPage.vue';

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
    {
        path: '/recipes',
        name: 'Recipes',
        component: RecipesListPage,
    },
    //   {
    //     path: '/recipes/:id',
    //     name: 'RecipeDetail',
    //     component: RecipeDetail,
    //     props: true  // Pass route params as props
    //   },
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

router.beforeEach(async (to, from) => {
    const auth = useUserStore();
    if (!auth.isLoggedIn && from.name === undefined) {
        await auth.hydrateUser();
    }

    // Allow users to login when not yet logged in.
    if (!auth.isLoggedIn && to.name === 'Login') {
        return true;
    }

    if (auth.isLoggedIn && to.name === 'Login') {
        return false;
    }

    // Force users to login to use the site.
    if (!auth.isLoggedIn) {
        console.warn('Redirecting non-logged-in user to Login');
        return { name: 'Login', replace: true };
    }

    // Redirect non-admin users away from admin pages.
    if (!auth.currentUser?.isAdmin && to.meta.requiresAdmin) {
        console.warn('Redirecting non-admin user to dashboard');
        return { name: 'Dashboard', replace: true };
    }

    console.info('Allowing navigation');
    return true;
});

export default router;