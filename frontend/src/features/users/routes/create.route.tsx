import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { usersBaseRoute } from "./index.route";

export const usersCreateRoute = createRoute({
  getParentRoute: () => usersBaseRoute,
  path: "/create",
  component: lazyRouteComponent(() => import("@/features/users/pages/create_user")),
});
