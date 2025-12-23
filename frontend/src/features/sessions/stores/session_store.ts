import { create } from "zustand";
import { httpClient } from "@/lib/http_client";
import * as v from "valibot";
import { SessionSummarySchema, type SessionSummary } from "../types";

type SessionStore = {
  getSessionSummaries: () => Promise<SessionSummary[]>;
};

export const useSessionStore = create<SessionStore>(() => ({
  getSessionSummaries: async () => {
    const response = await httpClient.get("sessions").json();
    const sessions = v.parse(v.array(SessionSummarySchema), response);
    return sessions;
  },
}));
