<template>
    <v-app>
        <v-main>
            <v-app-bar :elevation="2">
                <template v-slot:prepend>
                    <v-app-bar-nav-icon @click="drawerOpen = !drawerOpen"></v-app-bar-nav-icon>
                </template>
                <template v-slot:append>
                    <div v-if="userStore.isLoggedIn" style="padding-right: 1rem">
                        {{ userStore.userFullName }}
                    </div>
                </template>
                <v-app-bar-title>Mainframe</v-app-bar-title>
            </v-app-bar>
            <v-navigation-drawer v-if="userStore.isLoggedIn" v-model="drawerOpen">
                <v-list-item link to="/dashboard" title="Dashboard" />
                <v-divider />
                <v-list-item link to="/recipes" title="Recipes" />
                <!-- <v-list-item link to="/shopping" title="Shopping Lists" /> -->
                <!-- <v-list-item link to="/calendar" title="Calendar" /> -->
                <!-- <v-list-item link to="/passwords" title="Passwords" /> -->
                <v-divider />
                <v-list-item v-if="userStore.currentUser?.isAdmin" link to="/administration" title="Administration" />
                <v-divider />
                <v-list-item v-if="!userStore.isLoggedIn" link to="/login" title="Login" />
                <v-list-item v-if="userStore.isLoggedIn" @click="logout" title="Logout" />
            </v-navigation-drawer>
            <v-navigation-drawer v-else v-model="drawerOpen">
                <v-list-item v-if="!userStore.isLoggedIn" link to="/login" title="Login" />
            </v-navigation-drawer>
            <router-view />
        </v-main>
    </v-app>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useUserStore } from './stores/user';
import { useRouter } from 'vue-router';
const userStore = useUserStore();
const drawerOpen = ref(true);
const router = useRouter();

async function logout() {
    await userStore.logout();
    await router.replace({ name: 'Login' });
}
</script>
