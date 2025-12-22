import { AuthenticatedUser, LoginResponseSchema, type LoginRequest } from "@/features/auth/types";
import { handleApiRequest } from "@/utils/apiHelpers";
import { httpClient } from "@/utils/httpClient";
import * as v from "valibot";

import { ApiError } from "@/utils/apiHelpers";
// @ts-ignore
import type { ValiError } from "valibot";
import { setAuthStore } from "../stores/authStore";

export interface IAuthService {
  /**
   * This should only be called once from the root of the app and is used to manage
   * application refreshes to update the store.
   * Get the current user details from the backend when logged in.
   * @throws {ApiError} When authentication fails or server returns an error
   * @throws {ValiError} When the server response doesn't match the expected schema.
   */
  initialize(): Promise<void>;
  /**
   * Sign in a user with username and password
   * @throws {ApiError} When authentication fails or server returns an error
   * @throws {ValiError} When the server response doesn't match the expected schema.
   */
  login(request: LoginRequest): Promise<void>;

  /**
   * Sign out the current user
   * @throws {ApiError} When the request fails
   */
  logout(): Promise<void>;
}

export const authService: IAuthService = {
  initialize: async () => {
    return handleApiRequest(async () => {
      try {
        const response = await httpClient.get("auth/me").json();
        const result = v.parse(LoginResponseSchema, response);
        const user = new AuthenticatedUser(result);

        setAuthStore({
          user,
          error: null,
          isInitializing: false,
          isLoggedIn: true,
        });
      } catch (error) {
        if (error instanceof ApiError && error.statusCode === 401) {
          // not authorized is okay, we're just not logged in yet.
          setAuthStore({
            user: null,
            isInitializing: false,
            error: null,
            isLoggedIn: false,
          });
          return;
        }

        const message =
          error instanceof ApiError ? error.message : "Error occurred getting user details";

        setAuthStore({
          error: message,
          isInitializing: false,
        });

        throw error;
      }
    });
  },
  login: async (request) => {
    return handleApiRequest(async () => {
      try {
        const response = await httpClient.post("auth/login", { json: request }).json();
        const loginResponse = v.parse(LoginResponseSchema, response);
        const user = new AuthenticatedUser(loginResponse);
        setAuthStore({
          user,
          error: null,
          isLoggedIn: true,
        });
      } catch (error) {
        const message = error instanceof ApiError ? error.message : "Login failed";

        setAuthStore({
          error: message,
          user: null,
          isLoggedIn: false,
        });

        throw error;
      }
    });
  },

  logout: async () => {
    return handleApiRequest(async () => {
      try {
        await httpClient.post("auth/logout");

        setAuthStore({
          user: null,
          error: null,
          isLoggedIn: false,
        });
      } catch (error) {
        const message = error instanceof ApiError ? error.message : "Logout failed";

        setAuthStore({ error: message });
        throw error;
      }
    });
  },
};
