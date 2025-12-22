import { Navigate, Outlet } from "react-router-dom";
import { useAuthStore } from "@/features/auth/stores/authStore";

export function RequireAuth() {
  const isLoggedIn = useAuthStore((s) => s.isLoggedIn);

  if (!isLoggedIn) {
    return <Navigate to="/login" replace />;
  }

  return <Outlet />;
}
