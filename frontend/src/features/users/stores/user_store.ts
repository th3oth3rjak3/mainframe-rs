import { create } from "zustand";
import { UserBaseSchema, type CreateUserRequest, type UserBase } from "../types";
import { httpClient } from "@/lib/http_client";
import * as z from "zod";

type UserStore = {
  getAllUsers: () => Promise<UserBase[]>;
  createUser: (request: CreateUserRequest) => Promise<void>;
};

export const useUserStore = create<UserStore>(() => ({
  getAllUsers: async () => {
    const response = await httpClient.get("users").json();
    return z.parse(z.array(UserBaseSchema), response);
  },
  createUser: async (request: CreateUserRequest) => {
    await httpClient.post("users", { json: request });
  },
}));
