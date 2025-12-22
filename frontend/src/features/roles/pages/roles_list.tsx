import { useRoleStore } from "@/features/roles/stores/role_store";
import { useEffect } from "react";
import type { Role } from "../types";

export default function RolesList() {
  const initializeRoleStore = useRoleStore((state) => state.initialize);
  const roles: Role[] = useRoleStore((state) => state.roles);

  useEffect(() => {
    initializeRoleStore();
  }, [initializeRoleStore]);

  return (
    <ul>
      {roles.map((role) => (
        <li>{role.name}</li>
      ))}
    </ul>
  );
}
