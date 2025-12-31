import { type CreateUserRequest } from "../types";
import { useState } from "react";
import { NewUserEmailTemplate } from "@/features/users/components/new_user_email_template";
import { useCreateUser } from "../queries";
import { getAllRolesQueryOptions } from "@/features/roles/queries";
import { useQuery } from "@tanstack/react-query";
import { CreateUserForm } from "../components/create_user_form";

export default function CreateUser() {
  const createUser = useCreateUser();
  const { data: roles } = useQuery(getAllRolesQueryOptions);
  const [validatedUser, setValidatedUser] = useState<CreateUserRequest | null>(null);

  const onSubmit = async (request: CreateUserRequest) => {
    await createUser.mutateAsync(request);
    setValidatedUser(request);
  };

  if (createUser.isError) {
    setValidatedUser(null);
    return <div>Error occurred creating the new user: {createUser.error.message}</div>;
  }

  if (validatedUser) {
    return <NewUserEmailTemplate newUser={validatedUser} />;
  }

  return (
    <CreateUserForm onSubmit={onSubmit} isPending={createUser.isPending} roles={roles ?? []} />
  );
}
