import type { QueryClient } from "@tanstack/react-query";
import ky from "ky";
import * as z from "zod";

/**
 * Error response schema matching the backend ErrorResponse struct
 */
export const ErrorResponseSchema = z.object({
  error: z.string(),
});

export type ErrorResponse = z.infer<typeof ErrorResponseSchema>;

let queryClient: QueryClient | null = null;

export function setHttpClientQueryClient(client: QueryClient) {
  queryClient = client;
}

/**
 * httpClient is used to make HTTP requests to the backend API.
 * All requests are automatically prefixed with '/api' and include credentials (cookies).
 *
 * @example
 * // POST to /api/auth/signout
 * await httpClient.post('auth/signout')
 *
 * @example
 * // GET from /api/users/me
 * const user = await httpClient.get('users/me').json()
 */
export const httpClient = ky.create({
  prefixUrl: "/api",
  credentials: "include",
  timeout: 10_000,
  hooks: {
    beforeError: [
      async (error) => {
        const { response } = error;
        if (response && response.body) {
          const body = await response.json();
          const errorResponse = ErrorResponseSchema.safeParse(body);
          if (errorResponse.error) {
            error.message = z.prettifyError(errorResponse.error);
          }
        }

        return error;
      },
    ],
    afterResponse: [
      async (_, _options, response) => {
        if (response.status === 401 && queryClient) {
          // Clear the user from cache on 401
          queryClient.setQueryData(['user'], null);
        }
      }
    ],
  },
});
