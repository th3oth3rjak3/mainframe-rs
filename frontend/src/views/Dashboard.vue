<template>
    <v-container>
        <v-row>
            <v-col cols="12" sm="6" md="4" v-for="card of cardProps">
                <AppCard :route="card.route" :title="card.title" :subtitle="card.subtitle" :icon="card.icon"
                    :color="card.color" />
            </v-col>
        </v-row>
    </v-container>
</template>

<script setup lang="ts">
import AppCard, { type AppCardProps } from '@/components/AppCard.vue';
import { computed, ref } from 'vue';

const recipeCount = ref(10);
// const activeListCount = ref(2);
// const upcomingEvents = ref(6)

const cardProps = computed<AppCardProps[]>(() => [
    { route: "/recipes", title: "Recipe Manager", subtitle: `${recipeCount.value} recipes`, icon: "mdi-food", color: "primary" },
    // { route: "/shopping", title: "Shopping Lists", subtitle: `${activeListCount.value} active lists`, icon: "mdi-cart", color: "success" },
    // { route: "/calendar", title: "Family Calendar", subtitle: `${upcomingEvents.value} events this week`, icon: "mdi-calendar", color: "warning" }
]);


</script>