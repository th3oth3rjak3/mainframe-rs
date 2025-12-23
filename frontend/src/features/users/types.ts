import * as v from "valibot";

export const UserBaseSchema = v.object({
  id: v.pipe(v.string(), v.uuid()),
  username: v.string(),
  email: v.string(),
  firstName: v.string(),
  lastName: v.string(),
  isDisabled: v.boolean(),
  lastLogin: v.nullable(
    v.pipe(
      v.string(),
      v.isoTimestamp(),
      v.transform((dateStr) => new Date(dateStr))
    )
  ),
});

export type UserBase = v.InferOutput<typeof UserBaseSchema>;
