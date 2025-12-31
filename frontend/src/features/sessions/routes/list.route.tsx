import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { administratorRoleRoute } from "@/routes/authorization/administrator.route";

export const sessionsListRoute = createRoute({
  getParentRoute: () => administratorRoleRoute,
  path: "/sessions",
  component: lazyRouteComponent(() => import("@/features/sessions/pages/sessions_list")),
});
