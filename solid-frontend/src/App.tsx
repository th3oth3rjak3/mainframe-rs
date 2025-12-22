import "@/App.css";
import Layout from "@/components/layout/layout";
import { RequireAuth } from "@/components/layout/require-auth";
import Login from "@/features/auth/pages/Login";
import Home from "@/pages/home";
import { Route, Router } from "@solidjs/router";

import { ColorModeProvider } from "@kobalte/core";
import { onMount, Show } from "solid-js";
import { authService } from "./features/auth/services/authService";
import { authStore } from "./features/auth/stores/authStore";

function App() {
  onMount(() => {
    authService.initialize();
  });

  return (
    <Show when={!authStore.isInitializing}>
      <ColorModeProvider initialColorMode="system">
        <Router>
          <Route path="/login" component={Login} />
          <Route component={RequireAuth}>
            <Route component={Layout}>
              <Route path="/" component={Home} />
            </Route>
          </Route>
        </Router>
      </ColorModeProvider>
    </Show>
  );
}

export default App;
