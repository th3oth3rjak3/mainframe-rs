import { useRoleStore } from "@/features/roles/stores/role_store";
import { useEffect } from "react";
import type { Role } from "../types";
import type { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "@/components/ui/data_table";
import { Button } from "@/components/ui/button";
import { ArrowUpDown } from "lucide-react";

const columns: ColumnDef<Role>[] = [
  {
    accessorFn: (item) => item.id,
    header: "ID",
  },
  {
    accessorKey: "name",
    header: ({ column }) => {
      return (
        <Button
          variant="ghost"
          onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
        >
          Name
          <ArrowUpDown className="ml-2 h-4 w-4" />
        </Button>
      );
    },
  },
];

export default function RolesList() {
  const initializeRoleStore = useRoleStore((state) => state.initialize);
  const roles: Role[] = useRoleStore((state) => state.roles);

  useEffect(() => {
    initializeRoleStore();
  }, [initializeRoleStore]);

  return <DataTable columns={columns} data={roles} filterColumnName="name" />;
}
