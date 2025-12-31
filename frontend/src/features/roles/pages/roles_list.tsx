import type { Role } from "../types";
import type { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "@/shared/ui/data_table";
import { Button } from "@/shared/ui/button";
import { ArrowUpDown } from "lucide-react";
import { PageHeader } from "@/shared/ui/page_header";
import { useQuery } from "@tanstack/react-query";
import { getAllRolesQueryOptions } from "../queries";

const columns: ColumnDef<Role>[] = [
  {
    accessorFn: (item) => item.id,
    header: "Id",
    enableColumnFilter: false,
  },
  {
    accessorKey: "name",
    meta: {
      label: "Name",
    },
    header: ({ column }) => {
      return (
        <Button
          type="button"
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
  const { data: roles } = useQuery(getAllRolesQueryOptions);

  return (
    <>
      <PageHeader title="Roles" description="All roles available in the system" />
      <DataTable columns={columns} data={roles ?? []} showColumnSelector filterable />
    </>
  );
}
