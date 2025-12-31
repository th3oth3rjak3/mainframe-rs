import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { authenticatedLayoutRoute } from "@/routes/layouts/protected.layout";

export const recipesListRoute = createRoute({
  getParentRoute: () => authenticatedLayoutRoute,
  path: "/recipes",
  component: lazyRouteComponent(() => import("@/features/recipes/pages/recipes_list")),
});
