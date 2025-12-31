import { queryOptions } from "@tanstack/react-query";
import { getAllSessions } from "@/features/sessions/api";

export const getAllSessionsQueryOptions = queryOptions({
  queryKey: ["sessions"],
  queryFn: getAllSessions,
  staleTime: 1 * 60 * 1000,
});
