import { Book, BookLock, ChevronUp, Home, User2 } from "lucide-react";

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useAuthStore } from "@/features/auth/authStore";
import { AuthenticatedUser } from "@/features/auth/types";
import { ROLES } from "@/features/roles/types";
import { toast } from "sonner";
import { ModeToggle } from "./mode-toggle";

type Icon = typeof Home;

interface IMenuItem {
  title: string,
  url: string,
  icon: Icon,
  canAccess: (user: AuthenticatedUser | null) => boolean,
}

// Menu items.
const items: IMenuItem[] = [
  {
    title: "Home",
    url: "/",
    icon: Home,
    canAccess: (_: AuthenticatedUser | null) => true,
  },
  {
    title: "Roles",
    url: "/roles",
    icon: BookLock,
    canAccess: (user: AuthenticatedUser | null) => user !== null && user.isAdmin,
  },
  {
    title: "Recipes",
    url: "/recipes",
    icon: Book,
    canAccess: (user: AuthenticatedUser | null) =>
      user !== null && (user.hasRole(ROLES.RecipeUser) || user.isAdmin),
  },
];

type AppSidebarProps = {
  variant: "inset" | "floating" | "sidebar" | undefined;
};

export default function AppSidebar({ variant }: AppSidebarProps) {
  const logout = useAuthStore((state) => state.logout);
  const user = useAuthStore((state) => state.user);

  const handleLogout = async () => {
    try {
      await logout();
      toast.success("Logout Successful");
    } catch (error) {
      if (error instanceof Error) {
        toast.error(`Failed to logout: ${error.message}`);
      } else {
        toast.error("Failed to logout");
      }
    }
  };

  return (
    <Sidebar variant={variant}>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel className="text-md mb-2">Mainframe</SidebarGroupLabel>
          <ModeToggle />
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) =>
                item.canAccess(user) ? (
                  <SidebarMenuItem key={item.title}>
                    <SidebarMenuButton asChild>
                      <a href={item.url}>
                        <item.icon />
                        <span>{item.title}</span>
                      </a>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ) : null
              )}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <SidebarMenu>
          <SidebarMenuItem>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <SidebarMenuButton className="justify-center relative">
                  <User2 className="absolute left-2" />
                  <span>{user ? user.fullName : "Guest"}</span>
                  <ChevronUp className="absolute right-2 h-4 w-4" />
                </SidebarMenuButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent side="top" className="w-[var(--radix-popper-anchor-width)]">
                <DropdownMenuItem onClick={handleLogout}>
                  <span>Logout</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  );
}
