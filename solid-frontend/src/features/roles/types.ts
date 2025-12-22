import * as v from 'valibot';

/**
 * The only valid role names in the system.
 */
export const ROLES = {
    Administrator: "Administrator",
    BasicUser: "Basic User",
    RecipeUser: "Recipe User",
} as const;

/**
 * The schema for validating role names.
 */
export const RoleNameSchema = v.picklist(
    Object.values(ROLES)
);

/**
 * A limited set of specific roles that exist on the backend.
 */
export type RoleName = typeof ROLES[keyof typeof ROLES];

/**
 * Role schema for validation of roles
 */
export const RoleSchema = v.object({
    id: v.pipe(v.string(), v.uuid()),
    name: RoleNameSchema,
    // Add other role fields as needed
});

export type Role = v.InferOutput<typeof RoleSchema>;