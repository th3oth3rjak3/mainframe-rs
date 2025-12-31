import type { UserBase } from "../types";
import type { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "@/shared/ui/data_table";

import { Button } from "@/shared/ui/button";
import { PlusCircle } from "lucide-react";
import { PageHeader } from "@/shared/ui/page_header";
import { useNavigate } from "@tanstack/react-router";
import { getAllUsersQueryOptions } from "../queries";
import { useQuery } from "@tanstack/react-query";

const columns: ColumnDef<UserBase>[] = [
  {
    accessorKey: "firstName",
    header: "First Name",
  },
  {
    accessorKey: "lastName",
    header: "Last Name",
  },
  {
    accessorKey: "username",
    header: "Username",
  },
  {
    accessorKey: "email",
    header: "Email",
  },
  {
    accessorKey: "isDisabled",
    header: "Disabled",
    enableColumnFilter: false,
  },
  {
    accessorKey: "lastLogin",
    header: "Last Login",
    enableColumnFilter: false,
    cell: ({ row }) => {
      const loginDate = row.getValue("lastLogin") as Date | null;
      return loginDate?.toLocaleString();
    },
  },
];

export default function UsersList() {
  const { data: users } = useQuery(getAllUsersQueryOptions);
  const navigate = useNavigate();

  return (
    <>
      <PageHeader
        title="Users"
        description="A list of all user accounts in the system"
        actions={
          <Button
            type="button"
            variant="outline"
            onClick={() => navigate({ to: "/users/create" })}
            className="inline-flex items-center gap-x-2"
          >
            <PlusCircle className="h-4 w-4" />
            Create User
          </Button>
        }
      />
      <DataTable data={users ?? []} columns={columns} showColumnSelector filterable />
    </>
  );
}
