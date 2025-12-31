import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { publicLayoutRoute } from "@/routes/layouts/public.layout";

export const signUpRoute = createRoute({
  getParentRoute: () => publicLayoutRoute,
  path: "/sign-up",
  component: lazyRouteComponent(() =>
    import("@/features/auth/pages/sign_up").then((m) => ({ default: m.SignUp }))
  ),
});
