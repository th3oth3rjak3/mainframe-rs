import { create } from "zustand";
import { authService, type IAuthService } from "./authService";
import { AuthenticatedUser, type RoleName, type LoginRequest } from "./types";
// @ts-ignore
import { ApiError } from "@/lib/apiHelpers";

type AuthStore = {
  user: AuthenticatedUser | null;
  isInitializing: boolean;
  error: string | null;
  isLoggedIn: boolean;

  /**
   * Sign in a user with username and password
   * @throws {ApiError} When authentication fails
   */
  login: (request: LoginRequest, service?: IAuthService) => Promise<void>;

  /**
   * Sign out the current user
   */
  logout: (service?: IAuthService) => Promise<void>;

  /**
   * Check if a user is authenticated on mount
   */
  initialize: (service?: IAuthService) => Promise<void>;

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
  error: null,
  isLoggedIn: false,

  login: async (request, service = authService) => {
    try {
      const response = await service.signIn(request);
      const user = new AuthenticatedUser(response);
      set({ user, error: null, isLoggedIn: true });
    } catch (error) {
      const message = error instanceof ApiError ? error.message : "Login failed";
      set({ error: message, user: null });
      throw error;
    }
  },

  logout: async (service = authService) => {
    try {
      await service.signOut();
      set({ user: null, error: null, isLoggedIn: false });
    } catch (error) {
      const message = error instanceof ApiError ? error.message : "Logout failed";
      set({ error: message });
      throw error;
    }
  },

  initialize: async (service = authService) => {
    try {
      const response = await service.getCurrentUser();
      const user = new AuthenticatedUser(response);
      set({ user, error: null, isInitializing: false, isLoggedIn: true });
    } catch (error) {
      // It's not an error to not be logged in when checking the user status.
      if (error instanceof ApiError && error.statusCode === 401) {
        set({ user: null, isInitializing: false, error: null });
        return;
      }
      const message = error instanceof ApiError ? error.message : "Error occurred getting user details";
      set({ error: message });
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


