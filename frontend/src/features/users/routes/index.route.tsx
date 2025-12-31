import { createRoute } from "@tanstack/react-router";
import { authenticatedLayoutRoute } from "@/routes/layouts/protected.layout";

export const usersBaseRoute = createRoute({
  getParentRoute: () => authenticatedLayoutRoute,
  path: "/users",
});
