import { useEffect, useState } from "react";
import { useUserStore } from "../stores/user_store";
import type { UserBase } from "../types";
import { toast } from "sonner";
import type { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "@/components/ui/data_table";

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

  useEffect(() => {
    getAllUsers()
      .then((u) => setUsers(u))
      .catch((err) => {
        toast.error(JSON.stringify(err));
        console.error(err);
      });
  }, [getAllUsers]);

  return <DataTable data={users} columns={columns} showColumnSelector filterable />;
}
