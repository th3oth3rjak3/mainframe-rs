import { Book, BookLock, ChevronUp, UserRound, House } from "lucide-solid";

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
import { AuthenticatedUser } from "@/features/auth/types";
import { ROLES } from "@/features/roles/types";
import { toast } from "somoto";
import { ModeToggle } from "@/components/ui/mode-toggle";
import { A } from "@solidjs/router";
import { For, Show } from "solid-js";
import { authService } from "@/features/auth/services/authService";
import { authStore } from "@/features/auth/stores/authStore";

type Icon = typeof House;

interface IMenuItem {
  title: string;
  url: string;
  icon: Icon;
  canAccess: (user: AuthenticatedUser | null) => boolean;
}

// Menu items.
const items: IMenuItem[] = [
  {
    title: "Home",
    url: "/",
    icon: House,
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
  variant?: "inset" | "floating" | "sidebar";
};

export default function AppSidebar({ variant }: AppSidebarProps) {
  const handleLogout = async () => {
    try {
      await authService.logout();
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
          <SidebarGroupLabel class="text-md mb-2">Mainframe</SidebarGroupLabel>
          <ModeToggle />
          <SidebarGroupContent>
            <SidebarMenu>
              <For each={items}>
                {(item) => (
                  <Show when={item.canAccess(authStore.user)}>
                    <SidebarMenuItem>
                      <SidebarMenuButton as={A} href={item.url}>
                        <item.icon />
                        <span>{item.title}</span>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  </Show>
                )}
              </For>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <SidebarMenu>
          <SidebarMenuItem>
            <DropdownMenu>
              <DropdownMenuTrigger class="w-full">
                <SidebarMenuButton class="justify-center relative">
                  <UserRound class="absolute left-2" />
                  <span>{authStore.user ? authStore.user.fullName : "Guest"}</span>
                  <ChevronUp class="absolute right-2 h-4 w-4" />
                </SidebarMenuButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent class="w-[var(--kb-popper-anchor-width)]">
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
