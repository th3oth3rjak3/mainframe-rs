import {
  RouterProvider,
  createRoute,
  createRouter,
  notFound,
  type RegisteredRouter,
} from "@tanstack/react-router";
import { rootRoute, type RouterContext } from "@/routes/root";
import { publicLayoutRoute } from "@/routes/layouts/public.layout";
import { signUpRoute } from "@/features/auth/routes/sign_up";
import { forgotPasswordRoute } from "@/features/auth/routes/forgot_password";
import { loginRoute } from "@/features/auth/routes/login";
import { authenticatedLayoutRoute } from "@/routes/layouts/protected.layout";
import { rolesListRoute } from "@/features/roles/routes/list.route";
import { dashboardRoute } from "@/features/dashboard/routes/index.route";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { setHttpClientQueryClient } from "@/lib/http_client";
import { usersBaseRoute } from "@/features/users/routes/index.route";
import { usersListRoute } from "@/features/users/routes/list.route";
import { usersCreateRoute } from "@/features/users/routes/create.route";
import { sessionsListRoute } from "@/features/sessions/routes/list.route";
import { recipesListRoute } from "@/features/recipes/routes/list.route";
import { administratorRoleRoute } from "./authorization/administrator.route";

const catchAllProtectedLayoutRoute = createRoute({
  getParentRoute: () => authenticatedLayoutRoute,
  path: "/$",
  loader: () => {
    throw notFound();
  },
});

const routeTree = rootRoute.addChildren([
  publicLayoutRoute.addChildren([loginRoute, signUpRoute, forgotPasswordRoute]),
  authenticatedLayoutRoute.addChildren([
    catchAllProtectedLayoutRoute,
    dashboardRoute,
    recipesListRoute,
    administratorRoleRoute.addChildren([
      rolesListRoute,
      usersBaseRoute.addChildren([usersListRoute, usersCreateRoute]),
      sessionsListRoute,
    ]),
  ]),
]);

const queryClient = new QueryClient();
setHttpClientQueryClient(queryClient);

const router = createRouter({
  routeTree,
  context: {
    queryClient,
  },
  defaultNotFoundComponent: () => {
    return <p>Oops, we couldn't find what you were looking for!</p>;
  },
});

export function AppRouter() {
  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  );
}

export { type RouterContext };

export type ValidPaths =
  RegisteredRouter["routesByPath"][keyof RegisteredRouter["routesByPath"]]["fullPath"];

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
