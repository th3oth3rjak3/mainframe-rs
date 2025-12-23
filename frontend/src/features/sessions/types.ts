import * as v from "valibot";
import { UserBaseSchema } from "../users/types";

export const SessionSummarySchema = v.object({
  user: UserBaseSchema,
  activeSessions: v.number(),
});

export type SessionSummary = v.InferOutput<typeof SessionSummarySchema>;
