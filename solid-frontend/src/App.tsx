import "@/App.css";
import Layout from "@/components/layout/Layout";
import { RequireAuth } from "@/components/layout/RequireAuth";
import Login from "@/features/auth/pages/Login";
import { Route, Router } from "@solidjs/router";

import { ColorModeProvider } from "@kobalte/core";
import { lazy, onMount, Show } from "solid-js";
import { authService } from "@/features/auth/services/authService";
import { authStore } from "@/features/auth/stores/authStore";
import { Toaster } from "@/components/ui/sonner";

const Home = lazy(() => import("@/pages/home"));
const RolesList = lazy(() => import("@/features/roles/pages/RolesList"));

function App() {
  onMount(() => {
    authService.initialize();
  });

  return (
    <Show when={!authStore.isInitializing}>
      <ColorModeProvider initialColorMode="system">
        <Toaster />
        <Router>
          {/* Unprotected Routes */}
          <Route path="/login" component={Login} />

          {/* Protected Routes With Auth/Layout */}
          <Route component={RequireAuth}>
            <Route component={Layout}>
              <Route path="/" component={Home} />

              {/* Roles */}
              <Route path="/roles">
                <Route path="/" component={RolesList} />
              </Route>
            </Route>
          </Route>
        </Router>
      </ColorModeProvider>
    </Show>
  );
}

export default App;
