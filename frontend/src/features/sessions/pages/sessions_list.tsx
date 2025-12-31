import type { SessionSummary } from "../types";
import { DataTable } from "@/shared/ui/data_table";
import type { ColumnDef } from "@tanstack/react-table";
import { PageHeader } from "@/shared/ui/page_header";
import { useQuery } from "@tanstack/react-query";
import { getAllSessionsQueryOptions } from "../queries";

const columns: ColumnDef<SessionSummary>[] = [
  {
    accessorFn: (session) => session.user.firstName,
    id: "firstName",
    header: "First Name",
  },
  {
    accessorFn: (session) => session.user.lastName,
    id: "lastName",
    header: "Last Name",
  },
  {
    accessorFn: (session) => session.user.username,
    id: "username",
    header: "Username",
  },
  {
    accessorFn: (session) => session.user.email,
    id: "email",
    header: "Email",
  },
  {
    accessorFn: (session) => session.user.isDisabled,
    id: "disabled",
    header: "Disabled",
    enableColumnFilter: false,
  },
  {
    accessorFn: (session) => session.user.lastLogin,
    id: "lastLogin",
    header: "Last Login",
    enableColumnFilter: false,
    cell: ({ row }) => {
      const loginDate = row.getValue("lastLogin") as Date | null;
      return loginDate?.toLocaleString();
    },
  },
  {
    accessorKey: "activeSessions",
    header: "Active Sessions",
    enableColumnFilter: false,
  },
];

export default function SessionsList() {
  const { data: sessions, isLoading, isError, error } = useQuery(getAllSessionsQueryOptions);

  if (isLoading) {
    return <div>Loading sessions...</div>;
  }

  if (isError) {
    return <div>Error fetching sessions: {error.message}</div>;
  }

  return (
    <>
      <PageHeader title="Sessions" description="All active login sessions" />
      <DataTable data={sessions ?? []} columns={columns} showColumnSelector filterable />
    </>
  );
}
