import { Button } from "@/components/ui/button";
import { Controller, useForm } from "react-hook-form";
import { CreateUserRequestSchema, type CreateUserRequest } from "../types";
import dayjs from "dayjs";
import { useUserStore } from "../stores/user_store";
import { Field, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useEffect, useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { DatePicker } from "@/components/ui/date_picker";
import { DataTable } from "@/components/ui/data_table";
import { useRoleStore } from "@/features/roles/stores/role_store";
import type { ColumnDef } from "@tanstack/react-table";
import type { Role } from "@/features/roles/types";
import { ArrowUpDown, RefreshCw } from "lucide-react";
import { NewUserEmailTemplate } from "@/features/users/components/new_user_email_template";
import { generateRandomPassword } from "@/features/auth/password_utilities";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { toastErrorHandler } from "@/lib/error_handler";

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

export default function CreateUser() {
  const createUser = useUserStore((store) => store.createUser);
  const initializeRoleStore = useRoleStore((state) => state.initialize);
  const roles: Role[] = useRoleStore((state) => state.roles);
  const [isLoading, setIsLoading] = useState(false);
  const [selectedRoles, setSelectedRoles] = useState<Role[]>([]);
  const [validatedUser, setValidatedUser] = useState<CreateUserRequest | null>(null);

  useEffect(() => {
    initializeRoleStore();
  }, [initializeRoleStore]);

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

  const onSubmit = async (request: CreateUserRequest) => {
    try {
      setIsLoading(true);
      request.roles = selectedRoles.map((role) => role.id);
      await createUser(request);
      setValidatedUser(request);
    } catch (err) {
      toastErrorHandler(err);
      setValidatedUser(null);
    } finally {
      setIsLoading(false);
    }
  };

  const handleGeneratePassword = () => {
    const newPassword = generateRandomPassword();
    form.setValue("rawPassword", newPassword, { shouldValidate: true });
    form.setValue("confirmPassword", newPassword, { shouldValidate: true });
  };

  if (validatedUser) {
    return <NewUserEmailTemplate newUser={validatedUser} />;
  }

  return (
    <Card className="mx-auto max-w-4xl">
      <CardHeader>
        <CardTitle>Create New User</CardTitle>
        <CardDescription>Fill out the form below to create a new user account.</CardDescription>
      </CardHeader>
      <CardContent>
        <form id="create-user-form" onSubmit={form.handleSubmit(onSubmit)}>
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
                data={roles}
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
                disabled={validatedUser != null}
                className="inline-flex items-center gap-x-2"
              >
                Generate Password
                <RefreshCw className="h-4 w-4" />
              </Button>

              <Button type="submit" disabled={isLoading}>
                {isLoading ? "Submitting..." : "Submit"}
              </Button>
            </div>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
