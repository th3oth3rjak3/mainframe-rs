import { create } from "zustand";
import { UserBaseSchema, type UserBase } from "../types";
import { httpClient } from "@/lib/http_client";
import * as v from "valibot";

type UserStore = {
  getAllUsers: () => Promise<UserBase[]>;
};

export const useUserStore = create<UserStore>(() => ({
  getAllUsers: async () => {
    const response = await httpClient.get("users").json();
    const users = v.parse(v.array(UserBaseSchema), response);
    return users;
  },
}));
