<template>
    <v-container class="pa-8">
        <v-row justify="center">
            <v-col cols="12" sm="8" md="6" lg="4">
                <div class="text-center mb-8">
                    <h1 class="text-h3 mb-2">Login</h1>
                    <p class="text-grey-darken-1">Enter your credentials</p>
                </div>
                <LoginForm />
            </v-col>
        </v-row>
    </v-container>
</template>

<script setup lang="ts">
import LoginForm from '@/components/LoginForm.vue';
import { useUserStore } from '@/stores/user';
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
const userStore = useUserStore();
const router = useRouter();

onMounted(async () => {
    if (!userStore.isLoggedIn) {
        await userStore.hydrateUser();
        if (userStore.isLoggedIn) {
            await router.replace({ name: "Dashboard" });
        }
    }
});

</script>