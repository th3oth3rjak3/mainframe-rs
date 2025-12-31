import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { authenticatedLayoutRoute } from "@/routes/layouts/protected.layout";

export const dashboardRoute = createRoute({
  getParentRoute: () => authenticatedLayoutRoute,
  path: "/",
  component: lazyRouteComponent(() => import("@/features/dashboard/pages/dashboard")),
});
