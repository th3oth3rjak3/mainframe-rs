import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { usersBaseRoute } from "./index.route";

export const usersListRoute = createRoute({
  getParentRoute: () => usersBaseRoute,
  path: "/",
  component: lazyRouteComponent(() => import("@/features/users/pages/users_list")),
});
