import ThemeProvider from "@/shared/providers/theme_provider";
import { createRoute, Outlet } from "@tanstack/react-router";
import { rootRoute } from "../root";
import { Toaster } from "sonner";
import { Suspense } from "react";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

export const publicLayoutRoute = createRoute({
  getParentRoute: () => rootRoute,
  id: "public",
  component: () => (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <Toaster richColors />
      <Suspense fallback={<p>Loading...</p>}>
        <Outlet />
      </Suspense>
      <TanStackRouterDevtools />
    </ThemeProvider>
  ),
});
