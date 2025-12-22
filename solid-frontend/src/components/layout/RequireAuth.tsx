import { authStore } from "@/features/auth/stores/authStore";
import { Navigate } from "@solidjs/router";
import { type JSX } from "solid-js";

export function RequireAuth(props: { children?: JSX.Element }) {
  if (!authStore.user) {
    return <Navigate href="/login" />;
  }

  return props.children;
}
