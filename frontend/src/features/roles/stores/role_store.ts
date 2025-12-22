import { create } from "zustand";
import { RoleSchema, type Role } from "@/features/roles/types";
import { httpClient } from "@/lib/http_client";
import * as v from "valibot";

type RoleStore = {
  isInitializing: boolean;
  roles: Role[]

  /**
   * Roles never change in the database since they're strongly tied to the
   * access system, it's fine to fetch them once when the app loads, and then
   * not again until a page refresh.
   */
  initialize: () => Promise<void>,
};

export const useRoleStore = create<RoleStore>((set, get) => ({
    isInitializing: true,
    roles: [],

    initialize: async () => {
        const {isInitializing} = get();
        if (!isInitializing) {
            return;
        }

        const response = await httpClient.get("roles").json();
        const roles = v.parse(v.array(RoleSchema), response);
        set({ roles, isInitializing: false });
    }
}));