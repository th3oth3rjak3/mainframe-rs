import { Route, Routes } from "react-router-dom";
import { toast, Toaster } from "sonner";
import { useAuthStore } from "@/features/auth/stores/auth_store";
import { Suspense, useEffect, useRef } from "react";
import * as Pages from "@/routes";

function App() {
  const initialize = useAuthStore((state) => state.initialize);
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const isLoading = useAuthStore((state) => state.isInitializing);
  const wasLoggedIn = useRef(isLoggedIn);

  useEffect(() => {
    initialize();
  }, [initialize]);

  useEffect(() => {
    // Check if the user was just logged out
    if (wasLoggedIn.current && !isLoggedIn) {
      // ITS ONLY JOB: Show a toast to explain what happened.
      toast.error("Your session has expired.", {
        description: "Please log in again to continue.",
      });
    }

    // Update the ref for the next render
    wasLoggedIn.current = isLoggedIn;
  }, [isLoggedIn]);

  if (isLoading) {
    return <></>;
  }

  return (
    <Pages.ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <Toaster richColors />
      <Suspense>
        <Routes>
          {/* Unprotected Routes */}
          <Route path="/login" element={<Pages.Login />} />
          <Route path="/sign-up" element={<Pages.SignUp />} />
          <Route path="/forgot-password" element={<Pages.ForgotPassword />} />

          {/* Authenticated Routes */}
          <Route element={<Pages.RequireAuth />}>
            <Route element={<Pages.Layout />}>
              <Route path="/" element={<Pages.Dashboard />} />

              {/* Roles */}
              <Route path="/roles">
                <Route path="" element={<Pages.RolesList />} />
              </Route>

              {/* Users */}
              <Route path="/users">
                <Route path="" element={<Pages.UsersList />} />
                <Route path="create" element={<Pages.CreateUser />} />
              </Route>

              {/* Recipes */}
              <Route path="/recipes">
                <Route path="" element={<Pages.RecipesList />} />
              </Route>

              {/* Sessions */}
              <Route path="/sessions">
                <Route path="" element={<Pages.SessionsList />} />
              </Route>
            </Route>
          </Route>
        </Routes>
      </Suspense>
    </Pages.ThemeProvider>
  );
}

export default App;
