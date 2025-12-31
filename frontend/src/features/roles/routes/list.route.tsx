import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { administratorRoleRoute } from "@/routes/authorization/administrator.route";

export const rolesListRoute = createRoute({
  getParentRoute: () => administratorRoleRoute,
  path: "/roles",
  component: lazyRouteComponent(() =>
    import("@/features/roles/pages/roles_list").then((m) => ({ default: m.default }))
  ),
});
