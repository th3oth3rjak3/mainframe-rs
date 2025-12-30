import { lazy } from "react";

export const Login = lazy(() => import("@/features/auth/pages/login"));
export const SignUp = lazy(() => import("@/features/auth/pages/sign_up"));
export const ForgotPassword = lazy(() => import("@/features/auth/pages/forgot_password"));
export const Dashboard = lazy(() => import("@/pages/dashboard"));
export const RolesList = lazy(() => import("@/features/roles/pages/roles_list"));
export const UsersList = lazy(() => import("@/features/users/pages/users_list"));
export const CreateUser = lazy(() => import("@/features/users/pages/create_user"));
export const RecipesList = lazy(() => import("@/features/recipes/pages/recipes_list"));
export const SessionsList = lazy(() => import("@/features/sessions/pages/sessions_list"));
export const ThemeProvider = lazy(() => import("@/components/providers/theme_provider"));
export const Layout = lazy(() => import("@/components/layout/layout"));
export const RequireAuth = lazy(() => import("@/components/layout/require_auth"));
