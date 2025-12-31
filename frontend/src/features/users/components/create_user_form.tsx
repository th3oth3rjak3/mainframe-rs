import { Button } from "@/shared/ui/button";
import { Controller, useForm } from "react-hook-form";
import { CreateUserRequestSchema, type CreateUserRequest } from "../types";
import dayjs from "dayjs";
import { Field, FieldError, FieldGroup, FieldLabel } from "@/shared/ui/field";
import { Input } from "@/shared/ui/input";
import { useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { DatePicker } from "@/shared/ui/date_picker";
import { DataTable } from "@/shared/ui/data_table";
import type { ColumnDef } from "@tanstack/react-table";
import type { Role } from "@/features/roles/types";
import { ArrowUpDown, RefreshCw } from "lucide-react";
import { generateRandomPassword } from "@/features/auth/password_utilities";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/ui/card";

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

interface CreateUserFormProps {
  onSubmit: (data: CreateUserRequest) => void;
  isPending: boolean;
  roles: Role[];
}

export function CreateUserForm({ onSubmit, isPending, roles }: CreateUserFormProps) {
  const [selectedRoles, setSelectedRoles] = useState<Role[]>([]);
  const form = useForm<CreateUserRequest>({
    resolver: zodResolver(CreateUserRequestSchema),
    defaultValues: {
      firstName: "",
      lastName: "",
      email: "",
      username: "",
      rawPassword: "",
      confirmPassword: "",
      passwordExpiration: dayjs().add(7, "days").toDate(),
      roles: [],
    },
  });

  const handleGeneratePassword = () => {
    const newPassword = generateRandomPassword();
    form.setValue("rawPassword", newPassword, { shouldValidate: true });
    form.setValue("confirmPassword", newPassword, { shouldValidate: true });
  };

  const handleSubmit = (request: CreateUserRequest) => {
    request.roles = selectedRoles.map((r) => r.id);
    onSubmit(request);
  };

  return (
    <Card className="mx-auto max-w-4xl">
      <CardHeader>
        <CardTitle>Create New User</CardTitle>
        <CardDescription>Fill out the form below to create a new user account.</CardDescription>
      </CardHeader>
      <CardContent>
        <form id="create-user-form" onSubmit={form.handleSubmit(handleSubmit)}>
          <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
            <FieldGroup>
              <Controller
                name="firstName"
                control={form.control}
                render={({ field, fieldState }) => (
                  <Field>
                    <FieldLabel htmlFor="create-user-form-firstname">First Name</FieldLabel>
                    <Input
                      {...field}
                      id="create-user-form-firstname"
                      aria-invalid={fieldState.invalid}
                    />
                    {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </FieldGroup>

            <FieldGroup>
              <Controller
                name="lastName"
                control={form.control}
                render={({ field, fieldState }) => (
                  <Field>
                    <FieldLabel htmlFor="create-user-form-lastname">Last Name</FieldLabel>
                    <Input
                      {...field}
                      id="create-user-form-lastname"
                      aria-invalid={fieldState.invalid}
                    />
                    {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </FieldGroup>

            <FieldGroup>
              <Controller
                name="email"
                control={form.control}
                render={({ field, fieldState }) => (
                  <Field>
                    <FieldLabel htmlFor="create-user-form-email">Email</FieldLabel>
                    <Input
                      {...field}
                      id="create-user-form-email"
                      aria-invalid={fieldState.invalid}
                    />
                    {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </FieldGroup>

            <FieldGroup>
              <Controller
                name="username"
                control={form.control}
                render={({ field, fieldState }) => (
                  <Field>
                    <FieldLabel htmlFor="create-user-form-username">Username</FieldLabel>
                    <Input
                      {...field}
                      id="create-user-form-username"
                      aria-invalid={fieldState.invalid}
                    />
                    {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </FieldGroup>

            <FieldGroup>
              <Controller
                name="rawPassword"
                control={form.control}
                render={({ field, fieldState }) => (
                  <Field>
                    <FieldLabel htmlFor="create-user-form-password">Password</FieldLabel>
                    <Input
                      {...field}
                      id="create-user-form-password"
                      type="text"
                      aria-invalid={fieldState.invalid}
                    />
                    {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </FieldGroup>

            <FieldGroup>
              <Controller
                name="confirmPassword"
                control={form.control}
                render={({ field, fieldState }) => (
                  <Field>
                    <FieldLabel htmlFor="create-user-form-confirmPassword">
                      Confirm Password
                    </FieldLabel>
                    <Input
                      {...field}
                      id="create-user-form-confirmPassword"
                      type="text"
                      aria-invalid={fieldState.invalid}
                    />
                    {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </FieldGroup>

            <div className="col-span-1 md:col-span-2">
              <FieldGroup>
                <Controller
                  name="passwordExpiration"
                  control={form.control}
                  render={({ field, fieldState }) => (
                    <Field>
                      <DatePicker
                        {...field}
                        label="Password Expiration"
                        onDatePicked={(date) => (field.value = date ?? new Date())}
                        initialValue={field.value}
                        aria-invalid={fieldState.invalid}
                      />
                      {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                    </Field>
                  )}
                />
              </FieldGroup>
            </div>

            <div className="col-span-1 md:col-span-2">
              <DataTable
                columns={columns}
                data={roles ?? []}
                selectable
                onSelectionsChanged={setSelectedRoles}
                title="Roles"
              />
            </div>

            <div className="col-span-1 flex w-full justify-end gap-x-4 md:col-span-2">
              <Button
                type="button"
                variant="outline"
                size="default"
                onClick={handleGeneratePassword}
                title="Generate Password"
                className="inline-flex items-center gap-x-2"
              >
                Generate Password
                <RefreshCw className="h-4 w-4" />
              </Button>

              <Button type="submit" disabled={isPending}>
                {isPending ? "Submitting..." : "Submit"}
              </Button>
            </div>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
