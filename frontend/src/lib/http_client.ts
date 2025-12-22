import ky from "ky";
import * as v from "valibot";

/**
 * Error response schema matching the backend ErrorResponse struct
 */
export const ErrorResponseSchema = v.object({
  error: v.string(),
});

export type ErrorResponse = v.InferOutput<typeof ErrorResponseSchema>;

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
          const errorResponse = v.safeParse(ErrorResponseSchema, body);
          if (errorResponse.success) {
            error.message = errorResponse.output.error;
          }
        }

        return error;
      }
    ]
  }
});
