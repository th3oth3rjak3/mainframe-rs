import { queryOptions, useMutation, useQueryClient } from "@tanstack/react-query";
import { login, logout, fetchCurrentUser } from "@/features/auth/api";
import { useNavigate } from "@tanstack/react-router";

export const currentUserQueryOptions = queryOptions({
    queryKey: ['user'],
    queryFn: fetchCurrentUser,
    staleTime: 5 * 60 * 1000,
});

export function useLogin() {
    const queryClient = useQueryClient();
    const navigate = useNavigate();

    return useMutation({
        mutationFn: login,
        onSuccess: (user) => {
            // Update the user query cache
            queryClient.setQueryData(currentUserQueryOptions.queryKey, user);
            navigate({ to: '/' });
        },
    });
}

export function useLogout() {
    const queryClient = useQueryClient();
    const navigate = useNavigate();

    return useMutation({
        mutationFn: logout,
        onSuccess: () => {
            // Clear the user from cache
            queryClient.setQueryData(currentUserQueryOptions.queryKey, null);
            navigate({ to: '/login' });
        },
    });
}