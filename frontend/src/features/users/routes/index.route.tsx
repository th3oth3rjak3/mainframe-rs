import { createRoute } from "@tanstack/react-router";
import { administratorRoleRoute } from "@/routes/authorization/administrator.route";

export const usersBaseRoute = createRoute({
  getParentRoute: () => administratorRoleRoute,
  path: "/users",
});
