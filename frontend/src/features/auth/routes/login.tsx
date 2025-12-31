import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { publicLayoutRoute } from "@/routes/layouts/public.layout";

export const loginRoute = createRoute({
  getParentRoute: () => publicLayoutRoute,
  path: "/login",
  component: lazyRouteComponent(() =>
    import("@/features/auth/pages/login").then((m) => ({ default: m.Login }))
  ),
});
