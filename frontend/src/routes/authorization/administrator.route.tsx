import { createRoute, notFound } from "@tanstack/react-router";
import { currentUserQueryOptions } from "@/features/auth/queries";
import { authenticatedLayoutRoute } from "../layouts/protected.layout";

export const administratorRoleRoute = createRoute({
  getParentRoute: () => authenticatedLayoutRoute,
  id: "administrator",
  loader: async ({ context }) => {
    const user = await context.queryClient.ensureQueryData(currentUserQueryOptions);

    if (user && !user.hasRole("Administrator")) {
      throw notFound();
    }
  },
});
