import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { authenticatedLayoutRoute } from "@/routes/layouts/protected.layout";

export const rolesListRoute = createRoute({
  getParentRoute: () => authenticatedLayoutRoute,
  path: "/roles",
  component: lazyRouteComponent(() =>
    import("@/features/roles/pages/roles_list").then((m) => ({ default: m.default }))
  ),
});
