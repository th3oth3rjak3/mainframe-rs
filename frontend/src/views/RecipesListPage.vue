<template>
    <v-container class="pa-8">
        <v-row justify="center">
            <v-col cols="12">
                <v-card>
                    <!-- Search bar in card title area -->
                    <v-card-title class="d-flex flex-column flex-sm-row align-start align-sm-center pa-4 ga-3">
                        <span class="text-h5">Recipes</span>
                        <v-spacer class="d-none d-sm-flex" />
                        <v-text-field v-model="searchQuery" label="Search recipes" prepend-inner-icon="mdi-magnify"
                            variant="outlined" density="compact" clearable hide-details single-line
                            class="search-field" />
                    </v-card-title>

                    <v-divider />

                    <!-- Data table -->
                    <v-data-table-server v-model:items-per-page="pageSize" :headers="headers" :items="serverItems"
                        :items-per-page-options="[10, 25, 50, 100]" @update:options="loadItems"
                        :items-length="totalItems" loading-text="Loading... Please wait" :loading="loading"
                        :search="search" hover class="clickable-rows" @click:row="handleRowClick" />
                </v-card>
            </v-col>
        </v-row>
    </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRecipeStore, type Recipe } from '@/stores/recipe';
import { useRouter } from 'vue-router';
import { watchDebounced } from '@vueuse/core';

const recipeStore = useRecipeStore();
const router = useRouter();

const currentPage = ref<number>(1);
const pageSize = ref<number>(25);
const serverItems = ref<Recipe[]>([]);
const loading = ref(true);
const totalItems = ref(0);
const searchQuery = ref('');
const search = ref('');

watchDebounced(
    searchQuery,
    (newValue) => {
        console.log('Searching for:', newValue);
        search.value = newValue;
    },
    { debounce: 500 }
);

const headers = ref([
    { title: 'Name', key: 'name', sortable: false },
    { title: 'Author', key: 'author', sortable: false },
    { title: 'Difficulty', key: 'difficulty', sortable: false },
    { title: 'Duration', key: 'estimatedDuration', sortable: false },
]);

interface TableProps {
    page: number;
    itemsPerPage: number;
    sortBy: string;
    search: string;
}

async function loadItems({ page, itemsPerPage, search }: TableProps) {
    loading.value = true;
    const results = await recipeStore.searchRecipes(page, itemsPerPage, search);
    if (results === null) {
        return;
    }
    serverItems.value = results.data;
    totalItems.value = results.total;
    currentPage.value = results.page;
    pageSize.value = results.pageSize;
    loading.value = false;
}

const handleRowClick = (event: any, row: any) => {
    router.push(`/recipes/${row.item.id}`);
};
</script>

<style scoped>
.search-field {
    width: 100%;
    max-width: 400px;
}

@media (min-width: 600px) {
    .search-field {
        min-width: 300px;
    }
}

.clickable-rows :deep(tbody tr:hover) {
    cursor: pointer;
}
</style>
