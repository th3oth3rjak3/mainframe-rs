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
import { Input } from "@/components/ui/input";
import { useAuthStore } from "@/features/auth/stores/auth_store";
import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useForm, Controller } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { LoginRequestSchema, type LoginRequest } from "@/features/auth/types";
import { toast } from "sonner";

import { Field, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field";
import { HTTPError } from "ky";
import { toastErrorHandler } from "@/lib/error_handler";

export default function Login() {
  const login = useAuthStore((state) => state.login);
  const navigate = useNavigate();

  const [isLoading, setIsLoading] = useState(false);

  const form = useForm<LoginRequest>({
    resolver: zodResolver(LoginRequestSchema),
    defaultValues: {
      username: "",
      password: "",
    },
  });

  const onSubmit = async (request: LoginRequest) => {
    setIsLoading(true);

    try {
      await login(request);
      navigate("/", { replace: true });
    } catch (err) {
      if (err instanceof HTTPError) {
        if (err.response.status === 401) {
          toast.error("invalid username or password");
          return;
        }
        toastErrorHandler(err);
      } else {
        toastErrorHandler(err, "unexpected login error");
      }
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form id="login-form" onSubmit={form.handleSubmit(onSubmit)}>
      <div className="flex min-h-screen items-center justify-center">
        <Card className="w-full max-w-sm">
          <CardHeader>
            <CardTitle>Login to your account</CardTitle>
            <CardDescription>Enter your username below to login to your account</CardDescription>
            <CardAction>
              <Link
                to="/sign-up"
                className="ml-auto inline-block text-sm underline-offset-4 hover:underline"
              >
                Sign Up
              </Link>
            </CardAction>
          </CardHeader>
          <CardContent>
            <div className="flex flex-col gap-6">
              <div className="grid gap-2">
                <FieldGroup>
                  <Controller
                    name="username"
                    control={form.control}
                    render={({ field, fieldState }) => (
                      <Field>
                        <FieldLabel htmlFor="login-form-username">Username</FieldLabel>
                        <Input
                          {...field}
                          id="login-form-username"
                          aria-invalid={fieldState.invalid}
                        />
                        {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                      </Field>
                    )}
                  />
                </FieldGroup>
              </div>
              <div className="grid gap-2">
                <FieldGroup>
                  <Controller
                    name="password"
                    control={form.control}
                    render={({ field, fieldState }) => (
                      <Field>
                        <div className="flex items-center">
                          <FieldLabel htmlFor="login-form-password">Password</FieldLabel>
                          <Link
                            to="/forgot-password"
                            className="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                          >
                            Forgot your password?
                          </Link>
                        </div>
                        <Input
                          {...field}
                          id="login-form-password"
                          aria-invalid={fieldState.invalid}
                          type="password"
                        />
                        {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
                      </Field>
                    )}
                  />
                </FieldGroup>
              </div>
            </div>
          </CardContent>
          <CardFooter className="flex-col gap-2">
            <Button type="submit" className="w-full hover:cursor-pointer" disabled={isLoading}>
              {isLoading ? "Logging in..." : "Login"}
            </Button>
          </CardFooter>
        </Card>
      </div>
    </form>
  );
}
