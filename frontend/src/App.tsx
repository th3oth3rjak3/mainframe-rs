import { ThemeProvider } from "@/components/providers/theme-provider";
import { Route, Routes } from "react-router-dom";
import Layout from "@/components/layout/layout";
import { Toaster } from "sonner";
import Dashboard from "@/pages/dashboard";
import Login from "@/pages/login";
import { useAuthStore } from "./features/auth/authStore";
import { useEffect } from "react";
import SignUp from "./pages/sign-up";
import ForgotPassword from "./pages/forgot-password";
import { RequireAuth } from "./components/layout/require-auth";
import RolesList from "./pages/roles/roles-list";

function App() {
  const initialize = useAuthStore((state) => state.initialize);
  const isLoading = useAuthStore((state) => state.isInitializing);

  useEffect(() => {
    initialize();
  }, [initialize]);

  if (isLoading) {
    return <></>;
  }

  return (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <Toaster richColors />

      <Routes>
        <Route path="/login" element={<Login />} />
        <Route path="/sign-up" element={<SignUp />} />
        <Route path="/forgot-password" element={<ForgotPassword />} />

        <Route element={<RequireAuth />}>
          <Route element={<Layout />}>
            <Route path="/" element={<Dashboard />} />
            <Route path="/roles" element={<RolesList />} />
          </Route>
        </Route>
      </Routes>
    </ThemeProvider>
  );
}

export default App;
