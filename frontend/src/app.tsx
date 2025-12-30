import { Route, Routes } from "react-router-dom";
import { Toaster } from "sonner";
import { useAuthStore } from "@/features/auth/stores/auth_store";
import { lazy, Suspense, useEffect } from "react";

function App() {
  const initialize = useAuthStore((state) => state.initialize);
  const isLoading = useAuthStore((state) => state.isInitializing);

  const ThemeProvider = lazy(() => import("@/components/providers/theme_provider"));
  const Layout = lazy(() => import("@/components/layout/layout"));
  const RequireAuth = lazy(() => import("@/components/layout/require_auth"));
  const Login = lazy(() => import("@/features/auth/pages/login"));
  const SignUp = lazy(() => import("@/features/auth/pages/sign_up"));
  const ForgotPassword = lazy(() => import("@/features/auth/pages/forgot_password"));
  const Dashboard = lazy(() => import("@/pages/dashboard"));
  const RolesList = lazy(() => import("@/features/roles/pages/roles_list"));
  const UsersList = lazy(() => import("@/features/users/pages/users_list"));
  const CreateUser = lazy(() => import("@/features/users/pages/create_user"));
  const RecipesList = lazy(() => import("@/features/recipes/pages/recipes_list"));
  const SessionsList = lazy(() => import("@/features/sessions/pages/sessions_list"));

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

              {/* Users */}
              <Route path="/users">
                <Route path="" element={<UsersList />} />
                <Route path="create" element={<CreateUser />} />
              </Route>

              {/* Recipes */}
              <Route path="/recipes">
                <Route path="" element={<RecipesList />} />
              </Route>

              <Route path="/sessions">
                <Route path="" element={<SessionsList />} />
              </Route>
            </Route>
          </Route>
        </Routes>
      </Suspense>
    </ThemeProvider>
  );
}

export default App;
