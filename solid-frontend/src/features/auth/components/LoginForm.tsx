import { Button } from "@/components/ui/button/Button";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card/Card";
import { TextField, TextFieldInput, TextFieldLabel } from "@/components/ui/fields/TextField";
import { ApiError } from "@/utils/apiHelpers";
import { A, useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";
import { authService } from "../services/authService";

export default function LoginForm() {
  const navigate = useNavigate();

  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [error, setError] = createSignal<string | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);

  const onSubmit = async (e: Event) => {
    e.preventDefault();

    if (!username() || !password()) {
      setError("Username and password are required");
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      await authService.login({ username: username(), password: password() });
      navigate("/", { replace: true });
    } catch (err) {
      if (err instanceof ApiError) {
        setError(err.message);
      } else {
        setError("Invalid credentials");
      }
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={onSubmit}>
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
                <TextField>
                  <TextFieldLabel>Username</TextFieldLabel>
                  <TextFieldInput
                    type="text"
                    required
                    value={username()}
                    onChange={(e) => setUsername(e.target.value)}
                  ></TextFieldInput>
                </TextField>
              </div>
              <div class="grid gap-2">
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
                    required
                    value={password()}
                    onChange={(e) => setPassword(e.target.value)}
                  />
                </TextField>
              </div>
            </div>
            {error() && <p class="text-sm text-destructive">{error()}</p>}
          </CardContent>
          <CardFooter class="flex-col gap-2">
            <Button type="submit" class="w-full hover:cursor-pointer" disabled={isLoading()}>
              {isLoading() ? "Logging in..." : "Login"}
            </Button>
          </CardFooter>
        </Card>
      </div>
    </form>
  );
}
