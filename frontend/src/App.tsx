import { ThemeProvider } from "@/components/providers/theme_provider";
import { Route, Routes } from "react-router-dom";
import Layout from "@/components/layout/layout";
import { Toaster } from "sonner";
import Login from "@/features/auth/pages/login";
import { useAuthStore } from "@/features/auth/stores/auth_store";
import { lazy, Suspense, useEffect } from "react";
import SignUp from "./features/auth/pages/sign_up";
import ForgotPassword from "./features/auth/pages/forgot_password";
import { RequireAuth } from "./components/layout/require_auth";

function App() {
  const initialize = useAuthStore((state) => state.initialize);
  const isLoading = useAuthStore((state) => state.isInitializing);

  const Dashboard = lazy(() => import("@/pages/dashboard"));
  const RolesList = lazy(() => import("@/features/roles/pages/roles_list"));

  useEffect(() => {
    initialize();
  }, [initialize]);

  if (isLoading) {
    return <></>;
  }

  return (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <Toaster richColors />
      <Suspense>
        <Routes>
          {/* Unprotected Routes */}
          <Route path="/login" element={<Login />} />
          <Route path="/sign-up" element={<SignUp />} />
          <Route path="/forgot-password" element={<ForgotPassword />} />

          {/* Authenticated Routes */}
          <Route element={<RequireAuth />}>
            <Route element={<Layout />}>
              <Route path="/" element={<Dashboard />} />

              {/* Roles */}
              <Route path="/roles">
                <Route path="" element={<RolesList />} />
              </Route>
            </Route>
          </Route>
        </Routes>
      </Suspense>
    </ThemeProvider>
  );
}

export default App;
