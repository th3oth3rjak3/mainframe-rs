import { useEffect, useState } from "react";
import { useUserStore } from "../stores/user_store";
import type { UserBase } from "../types";
import type { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "@/components/ui/data_table";
import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { PlusCircle } from "lucide-react";
import { PageHeader } from "@/components/ui/page_header";
import { toastErrorHandler } from "@/lib/error_handler";

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
  const getAllUsers = useUserStore((store) => store.getAllUsers);
  const [users, setUsers] = useState<UserBase[]>([]);
  const navigate = useNavigate();

  useEffect(() => {
    getAllUsers()
      .then((u) => setUsers(u))
      .catch((err) => toastErrorHandler(err, "Error getting users"));
  }, [getAllUsers]);

  return (
    <>
      <PageHeader
        title="Users"
        description="A list of all user accounts in the system"
        actions={
          <Button
            type="button"
            variant="outline"
            onClick={() => navigate("/users/create")}
            className="inline-flex items-center gap-x-2"
          >
            <PlusCircle className="h-4 w-4" />
            Create User
          </Button>
        }
      />
      <DataTable data={users} columns={columns} showColumnSelector filterable />
    </>
  );
}
