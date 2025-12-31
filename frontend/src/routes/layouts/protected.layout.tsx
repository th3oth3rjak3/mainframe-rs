import ThemeProvider from "@/shared/providers/theme_provider";
import { createRoute, Outlet, redirect } from "@tanstack/react-router";
import { rootRoute } from "../root";
import { Toaster } from "sonner";
import { Suspense } from "react";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { SidebarInset, SidebarProvider } from "@/shared/ui/sidebar";
import AppSidebar from "@/shared/layout/app_sidebar";
import { AppBar } from "@/shared/layout/appbar";
import { currentUserQueryOptions } from "@/features/auth/queries";

export const authenticatedLayoutRoute = createRoute({
  getParentRoute: () => rootRoute,
  id: "authenticated",
  beforeLoad: async ({ context }) => {
    const user = await context.queryClient.ensureQueryData(currentUserQueryOptions);
    if (!user) {
      throw redirect({ to: "/login", replace: true });
    }
  },
  component: () => (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <Toaster richColors />
      <Suspense fallback={<p>Loading...</p>}>
        <SidebarProvider>
          <AppSidebar variant="floating" />
          <SidebarInset>
            <AppBar />
            <main className="p-4">
              <Outlet />
            </main>
          </SidebarInset>
        </SidebarProvider>
      </Suspense>
      <TanStackRouterDevtools position="bottom-right" />
    </ThemeProvider>
  ),
});
