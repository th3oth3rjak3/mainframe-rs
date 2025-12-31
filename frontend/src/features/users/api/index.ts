import { UserBaseSchema, type CreateUserRequest, type UserBase } from "@/features/users/types";
import { httpClient } from "@/lib/http_client";
import * as z from "zod";

export async function getAllUsers(): Promise<UserBase[]> {
    const response = await httpClient.get("users").json();
    return z.parse(z.array(UserBaseSchema), response);
}

export async function createUser(request: CreateUserRequest): Promise<void> {
    await httpClient.post("users", { json: request });
}