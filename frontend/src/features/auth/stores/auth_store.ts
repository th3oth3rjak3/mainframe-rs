import { create } from "zustand";
import { AuthenticatedUser, LoginResponseSchema, type LoginRequest } from "@/features/auth/types";
import type { RoleName } from "@/features/roles/types";
import { HTTPError } from "ky";
import * as v from "valibot";
import { httpClient } from "@/lib/http_client";

type AuthStore = {
  user: AuthenticatedUser | null;
  isInitializing: boolean;
  isLoggedIn: boolean;

  /**
   * Sign in a user with username and password
   */
  login: (request: LoginRequest) => Promise<void>;

  /**
   * Sign out the current user
   */
  logout: () => Promise<void>;

  /**
   * Check if a user is authenticated on mount
   */
  initialize: () => Promise<void>;

  /**
   * See if the current user has the required role.
   * @param name The name of the role to check for.
   * @returns True when the currently logged in user has the required role, otherwise false.
   */
  hasRole: (name: RoleName) => boolean;
};

export const useAuthStore = create<AuthStore>((set, get) => ({
  user: null,
  isInitializing: true,
  isLoggedIn: false,

  login: async (request) => {
      const responseBody = await httpClient.post("auth/login", { json: request }).json();
      const response = v.parse(LoginResponseSchema, responseBody);
      const user = new AuthenticatedUser(response);
      set({ user, isLoggedIn: true });
  },

  logout: async () => {
      await httpClient.post("auth/logout");
      set({ user: null, isLoggedIn: false });
  },

  initialize: async () => {
    try {
      const responseBody = await httpClient.get("auth/me").json();
      const response = v.parse(LoginResponseSchema, responseBody);
      const user = new AuthenticatedUser(response);
      set({ user, isInitializing: false, isLoggedIn: true });
    } catch (error) {
      // It's not an error to not be logged in when checking the user status.
      if (error instanceof HTTPError && error.response.status === 401) {
        set({ user: null, isInitializing: false });
        return;
      }

      throw error;
    }
  },

  hasRole: (name: RoleName) => {
    const {user} = get();
    if (user === null) {
      return false;
    }

    return user.hasRole(name);
  }
}));


