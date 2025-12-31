import { queryOptions } from "@tanstack/react-query";
import { getAllRoles } from "@/features/roles/api";

export const getAllRolesQueryOptions = queryOptions({
  queryKey: ["roles"],
  queryFn: getAllRoles,
  staleTime: 30 * 60 * 1000,
});
