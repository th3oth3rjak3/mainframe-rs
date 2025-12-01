import { paginatedResponseSchema } from "@/validation/generic";
import { defineStore } from "pinia";

import * as v from 'valibot';
import { useToast } from "vue-toastification";

const IngredientSchema = v.object({
    id: v.number(),
    position: v.number(),
    description: v.string(),
});

const IngredientRequestSchema = v.object({
    position: v.number(),
    descrption: v.string(),
});

const InstructionSchema = v.object({
    id: v.number(),
    position: v.number(),
    description: v.string(),
});

const InstructionRequestSchema = v.object({
    position: v.number(),
    description: v.string(),
});

const RecipeSchema = v.object({
    id: v.number(),
    name: v.string(),
    author: v.nullable(v.string()),
    description: v.nullable(v.string()),
    difficulty: v.nullable(v.string()),
    estimatedDuration: v.nullable(v.string()),
    userId: v.number(),
    isPublic: v.boolean(),
    ingredients: v.array(IngredientSchema),
    instructions: v.array(InstructionSchema),
});

const RecipeRequestSchema = v.object({
    name: v.string(),
    author: v.nullable(v.string()),
    description: v.nullable(v.string()),
    difficulty: v.nullable(v.string()),
    estimatedDuration: v.nullable(v.string()),
    isPublic: v.boolean(),
    ingredients: v.array(IngredientRequestSchema),
    instructions: v.array(InstructionRequestSchema),
});

export type Recipe = v.InferOutput<typeof RecipeSchema>;
export type Ingredient = v.InferOutput<typeof IngredientSchema>;
export type Instruction = v.InferOutput<typeof InstructionSchema>;

export type RecipeRequest = v.InferOutput<typeof RecipeRequestSchema>;
export type IngredientRequest = v.InferOutput<typeof IngredientRequestSchema>;
export type InstructionRequest = v.InferOutput<typeof InstructionRequestSchema>;

const PaginatedRecipeSchema = paginatedResponseSchema(RecipeSchema);
export type PaginatedRecipes = v.InferOutput<typeof PaginatedRecipeSchema>;

export const useRecipeStore = defineStore('recipe', {
    state: () => ({}),
    getters: {},
    actions: {
        async searchRecipes(page: number, pageSize: number, name?: string): Promise<PaginatedRecipes | null> {
            const toast = useToast();
            let query = `/api/recipes?page=${page}&pageSize=${pageSize}`;
            if (name) {
                query = query + `&q=${name}`;
            }

            const response = await fetch(query, {
                method: 'GET',
            });

            if (!response.ok) {
                toast.error('Error getting recipes from the server');
                return null;
            }

            const data = await response.json();
            const recipes = v.parse(paginatedResponseSchema(RecipeSchema), data);
            return recipes;
        }
    },
});