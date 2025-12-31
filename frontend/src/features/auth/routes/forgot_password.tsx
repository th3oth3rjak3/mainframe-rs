import { createRoute, lazyRouteComponent } from "@tanstack/react-router";
import { publicLayoutRoute } from "@/routes/layouts/public.layout";

export const forgotPasswordRoute = createRoute({
  getParentRoute: () => publicLayoutRoute,
  path: "/forgot-password",
  component: lazyRouteComponent(() =>
    import("@/features/auth/pages/forgot_password").then((m) => ({ default: m.ForgotPassword }))
  ),
});
