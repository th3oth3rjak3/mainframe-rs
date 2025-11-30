<template>
    <v-card @click="$router.push(route)" hover class="pa-4 text-center">
        <v-icon size="64" :color="props.color">{{ props.icon }}</v-icon>
        <v-card-title>{{ props.title }}</v-card-title>
        <v-card-subtitle v-if="props.subtitle">{{ props.subtitle }}</v-card-subtitle>
    </v-card>
</template>

<script setup lang="ts">
export interface AppCardProps {
    route: string;
    title: string;
    icon: string;
    color?: string;
    subtitle?: string;
};

const props = defineProps<AppCardProps>();
</script>