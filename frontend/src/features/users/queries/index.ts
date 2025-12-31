import { queryOptions, useMutation, useQueryClient } from "@tanstack/react-query";
import { createUser, getAllUsers } from "@/features/users/api";

export const getAllUsersQueryOptions = queryOptions({
  queryKey: ["users"],
  queryFn: getAllUsers,
  staleTime: 5 * 60 * 1000,
});

export function useCreateUser() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createUser,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["users"] });
    },
  });
}
