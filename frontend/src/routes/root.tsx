import type { QueryClient } from "@tanstack/react-query";
import { createRootRouteWithContext } from "@tanstack/react-router";

export type RouterContext = {
  queryClient: QueryClient;
};

export const rootRoute = createRootRouteWithContext<RouterContext>()({
  notFoundComponent: () => {
    return <p>Oops, we couldn't find what you were looking for!</p>;
  },
});
