import { handleApiRequest } from "@/lib/apiHelpers";
import { httpClient } from "@/lib/httpClient";
import * as v from "valibot";

// @ts-ignore
import type { ApiError } from "@/lib/apiHelpers";
// @ts-ignore
import type { ValiError } from "valibot";
import { RoleSchema, type Role } from "./types";

export interface IRoleService {
    /**
     * Get a list of all available roles.
     * @throws {ApiError} When authentication fails or server returns an error
     * @throws {ValiError} When the server response doesn't match the expected schema.
     */
    getAllRoles(): Promise<Role[]>;
}

export const roleService: IRoleService = {
    getAllRoles: async () => {
        return handleApiRequest(async () => {
            const response = await httpClient.get("roles").json();

            return v.parse(v.array(RoleSchema), response);
        });
    }
};
