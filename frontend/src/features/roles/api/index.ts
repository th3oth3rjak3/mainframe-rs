import { httpClient } from "@/lib/http_client";
import { RoleSchema, type Role } from "@/features/roles/types";

import * as z from "zod";

export async function getAllRoles(): Promise<Role[]> {
  const response = await httpClient.get("roles").json();
  const roles = z.parse(z.array(RoleSchema), response);
  return roles;
}
