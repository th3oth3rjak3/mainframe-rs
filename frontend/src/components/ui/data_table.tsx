import {
  type ColumnDef,
  type SortingState,
  type ColumnFiltersState,
  type VisibilityState,
  flexRender,
  getCoreRowModel,
  useReactTable,
  getSortedRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  type RowSelectionState,
  type Table as TanstackTable,
} from "@tanstack/react-table";

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown_menu";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

import { Button } from "@/components/ui/button";
import { useEffect, useMemo, useState } from "react";
import { Checkbox } from "./checkbox";
import { ClearableInput } from "./clearable_input";

interface DataTableProps<TData, TValue> {
  title?: string;
  description?: string;
  columns: ColumnDef<TData, TValue>[];
  data: TData[];
  filterable?: boolean;
  selectable?: boolean;
  onSelectionsChanged?: (selections: TData[]) => void | Promise<void>;
  showColumnSelector?: boolean;
}

interface ColumnMetadata {
  label?: string;
}

interface DataTableToolbarProps<TData> {
  table: TanstackTable<TData>;
  filterable?: boolean;
  showColumnSelector?: boolean;
}

function DataTableToolbar<TData>({
  table,
  filterable,
  showColumnSelector,
}: DataTableToolbarProps<TData>) {
  const filterableColumns = useMemo(
    () =>
      table
        .getAllColumns()
        .filter((column) => column.getCanFilter() && column.id !== "select")
        .map((column) => ({
          id: column.id,
          label:
            (column.columnDef.meta as { label: string })?.label ??
            (typeof column.columnDef.header === "string" ? column.columnDef.header : column.id),
        })),
    [table]
  );

  const [selectedFilterColumn, setSelectedFilterColumn] = useState<string>(() => {
    if (filterable && filterableColumns.length > 0) {
      return filterableColumns[0].id;
    }
    return "";
  });

  const selectedColumnLabel =
    filterableColumns.find((col) => col.id === selectedFilterColumn)?.label || selectedFilterColumn;

  if (!filterable && !showColumnSelector) {
    return null;
  }

  return (
    <div className="flex items-center gap-2 pt-4">
      {filterable && (
        <>
          <ClearableInput
            placeholder={`Filter by ${selectedColumnLabel}...`}
            value={
              selectedFilterColumn === ""
                ? ""
                : ((table.getColumn(selectedFilterColumn)?.getFilterValue() as string) ?? "")
            }
            onChange={(event) => {
              if (selectedFilterColumn && selectedFilterColumn !== "") {
                table.getColumn(selectedFilterColumn)?.setFilterValue(event.target.value);
              }
            }}
            onClear={() => table.getColumn(selectedFilterColumn)?.setFilterValue("")}
            className="max-w-sm"
          />
          <Select
            value={selectedFilterColumn}
            onValueChange={(value) => {
              if (selectedFilterColumn) {
                table.getColumn(selectedFilterColumn)?.setFilterValue("");
              }
              setSelectedFilterColumn(value);
            }}
          >
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Select column" />
            </SelectTrigger>
            <SelectContent>
              {filterableColumns.map((column) => (
                <SelectItem key={column.id} value={column.id}>
                  {column.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </>
      )}
      {showColumnSelector && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className="ml-auto">
              Columns
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            {table
              .getAllColumns()
              .filter((column) => column.getCanHide())
              .map((column) => (
                <DropdownMenuCheckboxItem
                  key={column.id}
                  className="capitalize"
                  checked={column.getIsVisible()}
                  onCheckedChange={(value) => column.toggleVisibility(!!value)}
                >
                  {(column.columnDef.meta as ColumnMetadata)?.label ?? column.id}
                </DropdownMenuCheckboxItem>
              ))}
          </DropdownMenuContent>
        </DropdownMenu>
      )}
    </div>
  );
}

interface DataTablePaginationProps<TData> {
  table: TanstackTable<TData>;
  selectable?: boolean;
}

function DataTablePagination<TData>({ table, selectable }: DataTablePaginationProps<TData>) {
  return (
    <div className="flex w-full items-center">
      <div className="text-muted-foreground text-sm">
        {selectable &&
          `${table.getFilteredSelectedRowModel().rows.length} of ${
            table.getFilteredRowModel().rows.length
          } row(s) selected.`}
      </div>

      <div className="ml-auto flex items-center space-x-2">
        <Button
          variant="outline"
          size="sm"
          onClick={() => table.previousPage()}
          disabled={!table.getCanPreviousPage()}
        >
          Previous
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={() => table.nextPage()}
          disabled={!table.getCanNextPage()}
        >
          Next
        </Button>
      </div>
    </div>
  );
}

export function DataTable<TData, TValue>({
  title,
  description,
  columns,
  data,
  filterable,
  selectable,
  onSelectionsChanged,
  showColumnSelector,
}: DataTableProps<TData, TValue>) {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState<RowSelectionState>({});

  const tableColumns = useMemo(() => {
    if (!selectable) return columns;

    const selectColumn: ColumnDef<TData, TValue> = {
      id: "select",
      header: ({ table }) => (
        <Checkbox
          checked={table.getIsAllPageRowsSelected()}
          onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
          aria-label="Select all"
        />
      ),
      cell: ({ row }) => (
        <Checkbox
          checked={row.getIsSelected()}
          onCheckedChange={(value) => row.toggleSelected(!!value)}
          aria-label="Select row"
        />
      ),
      enableSorting: false,
      enableHiding: false,
    };
    return [selectColumn, ...columns];
  }, [columns, selectable]);

  useEffect(() => {
    if (selectable && onSelectionsChanged) {
      const selectedData = Object.keys(rowSelection)
        .map(Number)
        .map((key) => data[key])
        .filter(Boolean); // Filter out potential undefined values
      onSelectionsChanged(selectedData);
    }
  }, [rowSelection, data, onSelectionsChanged, selectable]);

  const table = useReactTable({
    data,
    columns: tableColumns,
    state: { sorting, columnFilters, columnVisibility, rowSelection },
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
  });

  return (
    <Card>
      <CardHeader>
        {title && <CardTitle>{title}</CardTitle>}
        {description && <CardDescription>{description}</CardDescription>}
        <DataTableToolbar
          table={table}
          filterable={filterable}
          showColumnSelector={showColumnSelector}
        />
      </CardHeader>
      <CardContent>
        <div className="rounded-md border">
          <Table>
            <TableHeader>
              {table.getHeaderGroups().map((headerGroup) => (
                <TableRow key={headerGroup.id}>
                  {headerGroup.headers.map((header) => (
                    <TableHead key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(header.column.columnDef.header, header.getContext())}
                    </TableHead>
                  ))}
                </TableRow>
              ))}
            </TableHeader>
            <TableBody>
              {table.getRowModel().rows?.length ? (
                table.getRowModel().rows.map((row) => (
                  <TableRow key={row.id} data-state={row.getIsSelected() && "selected"}>
                    {row.getVisibleCells().map((cell) => (
                      <TableCell key={cell.id}>
                        {flexRender(cell.column.columnDef.cell, cell.getContext())}
                      </TableCell>
                    ))}
                  </TableRow>
                ))
              ) : (
                <TableRow>
                  <TableCell colSpan={tableColumns.length} className="h-24 text-center">
                    No results.
                  </TableCell>
                </TableRow>
              )}
            </TableBody>
          </Table>
        </div>
      </CardContent>
      <CardFooter>
        <DataTablePagination table={table} selectable={selectable} />
      </CardFooter>
    </Card>
  );
}
