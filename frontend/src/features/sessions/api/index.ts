import { httpClient } from "@/lib/http_client";
import { SessionSummarySchema, type SessionSummary } from "@/features/sessions/types";
import * as z from "zod";

export async function getAllSessions(): Promise<SessionSummary[]> {
  const response = await httpClient.get("sessions").json();
  return z.parse(z.array(SessionSummarySchema), response);
}
