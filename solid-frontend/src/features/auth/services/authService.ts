import { AuthenticatedUser, LoginResponseSchema, type LoginRequest } from "@/features/auth/types";
import { httpClient } from "@/utils/httpClient";
import * as v from "valibot";

// @ts-ignore
import type { ValiError } from "valibot";
import { setAuthStore } from "../stores/authStore";
import { HTTPError } from "ky";

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
    try {
      const response = await httpClient.get("auth/me").json();
      const result = v.parse(LoginResponseSchema, response);
      const user = new AuthenticatedUser(result);

      setAuthStore({
        user,
        isInitializing: false,
        isLoggedIn: true,
      });
    } catch (error) {
      if (error instanceof HTTPError && error.response.status === 401) {
        // not authorized is okay, we're just not logged in yet.
        setAuthStore({
          user: null,
          isInitializing: false,
          isLoggedIn: false,
        });

        return;
      }

      setAuthStore({
        isInitializing: false,
      });

      throw error;
    }
  },
  login: async (request) => {
    const response = await httpClient.post("auth/login", { json: request }).json();
    const loginResponse = v.parse(LoginResponseSchema, response);
    const user = new AuthenticatedUser(loginResponse);
    setAuthStore({
      user,
      isLoggedIn: true,
    });
  },
  logout: async () => {
    await httpClient.post("auth/logout");
    setAuthStore({
      user: null,
      isLoggedIn: false,
    });
  },
};
