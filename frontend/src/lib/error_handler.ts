import { HTTPError } from "ky";
import { toast } from "sonner";

/**
 * A centralized error handler for API calls in React components.
 * It shows a toast for any error EXCEPT for 401 Unauthorized,
 * which is handled globally by the session timeout logic.
 * @param error The error object caught in a try/catch block.
 * @param [customMessage] An optional custom message to display.
 */
export function toastErrorHandler(error: unknown, customMessage?: string) {
  if (error instanceof HTTPError && error.response.status === 401) {
    // This is a session timeout. Do nothing.
    // The global system in App.tsx will show the "Session Expired" toast.
    return;
  }

  // For all other errors, show a generic or custom toast.
  const message = customMessage || "An unexpected error occurred. Please try again.";
  toast.error(message, {
    description: error instanceof Error ? error.message : undefined,
  });
}
