import { AuthenticatedUser, LoginResponseSchema, type LoginRequest } from "@/features/auth/types";
import { httpClient } from "@/lib/http_client";
import { HTTPError } from "ky";
import * as z from "zod";

export async function fetchCurrentUser(): Promise<AuthenticatedUser | null> {
  try {
    const responseBody = await httpClient.get("auth/me").json();
    const response = z.parse(LoginResponseSchema, responseBody);
    const user = new AuthenticatedUser(response);
    return user;
  } catch (error) {
    // It's not an error to not be logged in when checking the user status.
    if (error instanceof HTTPError && error.response.status === 401) {
      return null;
    }

    throw error;
  }
}

export async function login(request: LoginRequest): Promise<AuthenticatedUser | null> {
  const responseBody = await httpClient.post("auth/login", { json: request }).json();
  const response = z.parse(LoginResponseSchema, responseBody);
  const user = new AuthenticatedUser(response);
  return user;
}

export async function logout(): Promise<void> {
  await httpClient.post("auth/logout");
}
