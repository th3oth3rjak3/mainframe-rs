import { ROLES, RoleSchema, type Role, type RoleName } from "@/features/roles/types";
import * as v from "valibot";

/**
 * User schema for validation at the API boundary matching the backend UserRead struct.
 * Timestamps are transformed from ISO strings to Date objects.
 */
export const UserSchema = v.object({
  id: v.pipe(v.string(), v.uuid()),
  username: v.string(),
  email: v.pipe(v.string(), v.email()),
  firstName: v.string(),
  lastName: v.string(),
  lastLogin: v.nullable(
    v.pipe(
      v.string(),
      v.isoTimestamp(),
      v.transform((isoStr) => new Date(isoStr))
    )
  ),
  failedLoginAttempts: v.number(),
  lastFailedLoginAttempt: v.nullable(
    v.pipe(
      v.string(),
      v.isoTimestamp(),
      v.transform((isoStr) => new Date(isoStr))
    )
  ),
  isDisabled: v.boolean(),
  createdAt: v.pipe(
    v.string(),
    v.isoTimestamp(),
    v.transform((isoStr) => new Date(isoStr))
  ),
  updatedAt: v.pipe(
    v.string(),
    v.isoTimestamp(),
    v.transform((isoStr) => new Date(isoStr))
  ),
  roles: v.array(RoleSchema),
});

export type User = v.InferOutput<typeof UserSchema>;

/**
 * Login response schema matching the backend LoginResponse struct.
 * Note: This is a simpler representation returned on login.
 */
export const LoginResponseSchema = v.object({
  username: v.string(),
  email: v.pipe(v.string(), v.email()),
  firstName: v.string(),
  lastName: v.string(),
  lastLogin: v.nullable(
    v.pipe(
      v.string(),
      v.isoTimestamp(),
      v.transform((str) => new Date(str))
    )
  ),
  roles: v.array(RoleSchema),
});

export type LoginResponse = v.InferOutput<typeof LoginResponseSchema>;

/**
 * Sign in request payload
 */
export type LoginRequest = {
  username: string;
  password: string;
};

/**
 * AuthenticatedUser is the details of a user who has been successfully
 * logged into the application. The only way to create one is by getting
 * a successful login response and passing that in the constructor.
 */
export class AuthenticatedUser {
  username: string;
  email: string;
  firstName: string;
  lastName: string;
  lastLogin: Date | null;
  roles: Role[];

  constructor(loginResponse: LoginResponse) {
    this.username = loginResponse.username;
    this.email = loginResponse.email;
    this.firstName = loginResponse.firstName;
    this.lastName = loginResponse.lastName;
    this.lastLogin = loginResponse.lastLogin;
    this.roles = loginResponse.roles;
  }

  hasRole(name: RoleName): boolean {
    return this.roles.map(r => r.name).includes(name);
  }

  get isAdmin(): boolean {
    return this.hasRole(ROLES.Administrator);
  }

  get fullName(): string {
    return `${this.firstName} ${this.lastName}`;
  }
}