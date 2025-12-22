import { AuthenticatedUser } from "@/features/auth/types";
import { createStore } from "solid-js/store";

type AuthStore = {
  user: AuthenticatedUser | null;
  isInitializing: boolean;
  error: string | null;
  isLoggedIn: boolean;
};

export const [authStore, setAuthStore] = createStore<AuthStore>({
  user: null,
  isInitializing: true,
  error: null,
  isLoggedIn: false,
});
