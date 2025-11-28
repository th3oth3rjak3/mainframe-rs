import { defineStore } from "pinia";

import * as v from 'valibot';

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

type Recipe = v.InferOutput<typeof RecipeSchema>;
type Ingredient = v.InferOutput<typeof IngredientSchema>;
type Instruction = v.InferOutput<typeof InstructionSchema>;

type RecipeRequest = v.InferOutput<typeof RecipeRequestSchema>;
type IngredientRequest = v.InferOutput<typeof IngredientRequestSchema>;
type InstructionRequest = v.InferOutput<typeof InstructionRequestSchema>;

export const useRecipeStore = defineStore('recipe', {
    state: () => ({}),
    getters: {},
    actions: {},
});