import { Button } from "@/components/ui/button";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { TextField, TextFieldInput, TextFieldLabel } from "@/components/ui/fields/TextField";
import { A, useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";
import { authService } from "@/features/auth/services/authService";
import { HTTPError } from "ky";
import { toast } from "somoto";
import { createForm } from "@tanstack/solid-form";
import * as v from "valibot";

export default function LoginForm() {
  const navigate = useNavigate();
  const [isLoading, setIsLoading] = createSignal(false);

  const onSubmit = async (username: string, password: string) => {
    setIsLoading(true);

    try {
      await authService.login({ username, password });
      navigate("/", { replace: true });
    } catch (err) {
      if (err instanceof HTTPError) {
        toast.error(err.message);
      } else {
        toast.error("Invalid credentials");
      }
    } finally {
      setIsLoading(false);
    }
  };

  const form = createForm(() => ({
    defaultValues: {
      username: "",
      password: "",
    },
    onSubmit: async ({ value }) => {
      await onSubmit(value.username, value.password);
    },
  }));

  return (
    <div class="flex min-h-screen items-center justify-center">
      <Card class="w-full max-w-sm">
        <CardHeader>
          <CardTitle>Login to your account</CardTitle>
          <CardDescription>Enter your username below to login to your account</CardDescription>
          <CardAction>
            <A
              href="/sign-up"
              class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
            >
              Sign Up
            </A>
          </CardAction>
        </CardHeader>
        <CardContent>
          <div class="flex flex-col gap-6">
            <div class="grid gap-2">
              <form.Field
                name="username"
                validators={{
                  onChange: ({ value }) => {
                    const result = v.safeParse(
                      v.pipe(
                        v.string(),
                        v.minLength(3, "Username must be at least three characters long"),
                        v.maxLength(50, "Username must be less than 50 characters long")
                      ),
                      value
                    );

                    return result.success
                      ? undefined
                      : result.issues.map((issue) => issue.message).join(", ");
                  },
                }}
              >
                {(field) => (
                  <>
                    <TextField>
                      <TextFieldLabel>Username</TextFieldLabel>
                      <TextFieldInput
                        type="text"
                        name={field().name}
                        required
                        value={field().state.value}
                        onChange={(e) => field().handleChange(e.target.value)}
                      ></TextFieldInput>
                    </TextField>
                    {!field().state.meta.isValid ? (
                      <small class="text-destructive" role="alert">
                        {field().state.meta.errors.join(", ")}
                      </small>
                    ) : null}
                  </>
                )}
              </form.Field>
            </div>
            <div class="grid gap-2">
              <form.Field
                name="password"
                validators={{
                  onChange: v.pipe(
                    v.string(),
                    v.minLength(8, "password must be at least 8 characters long"),
                    v.maxLength(50, "password must be less than 50 characters long")
                  ),
                }}
              >
                {(field) => (
                  <>
                    <TextField>
                      <div class="flex items-center">
                        <TextFieldLabel>Password</TextFieldLabel>
                        <A
                          href="/forgot-password"
                          class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                        >
                          Forgot your password?
                        </A>
                      </div>
                      <TextFieldInput
                        type="password"
                        name={field().name}
                        required
                        value={field().state.value}
                        onChange={(e) => field().handleChange(e.target.value)}
                      />
                    </TextField>
                    {!field().state.meta.isValid ? (
                      <small class="text-destructive" role="alert">
                        {field().state.meta.errors.join(", ")}
                      </small>
                    ) : null}
                  </>
                )}
              </form.Field>
            </div>
          </div>
        </CardContent>
        <CardFooter class="flex-col gap-2">
          <Button type="submit" class="w-full hover:cursor-pointer" disabled={isLoading()}>
            {isLoading() ? "Logging in..." : "Login"}
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
