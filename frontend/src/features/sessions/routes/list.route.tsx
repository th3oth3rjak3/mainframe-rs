import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { authenticatedLayoutRoute } from "@/routes/layouts/protected.layout";

export const sessionsListRoute = createRoute({
  getParentRoute: () => authenticatedLayoutRoute,
  path: "/sessions",
  component: lazyRouteComponent(() => import("@/features/sessions/pages/sessions_list")),
});
