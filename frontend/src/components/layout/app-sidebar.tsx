import { Book, Calendar, ChevronUp, Home, Inbox, Search, Settings, User2 } from "lucide-react";

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
import { ModeToggle } from "./mode-toggle";
import { useAuthStore } from "@/features/auth/authStore";
import { toast } from "sonner";
import { ROLES } from "@/features/auth/types";

// Menu items.
const items = [
  {
    title: "Home",
    url: "#",
    icon: Home,
    role: null,
  },
  {
    title: "Inbox",
    url: "#",
    icon: Inbox,
    role: null,
  },
  {
    title: "Calendar",
    url: "#",
    icon: Calendar,
    role: null,
  },
  {
    title: "Recipes",
    url: "#",
    icon: Book,
    role: ROLES.RecipeUser,
  },
  {
    title: "Search",
    url: "#",
    icon: Search,
    role: null,
  },
  {
    title: "Settings",
    url: "#",
    icon: Settings,
    role: null,
  },
];

type AppSidebarProps = {
  variant: "inset" | "floating" | "sidebar" | undefined;
};

export default function AppSidebar({ variant }: AppSidebarProps) {
  const logout = useAuthStore((state) => state.logout);
  const hasRole = useAuthStore((state) => state.hasRole);

  const handleSignOut = async () => {
    try {
      await logout();
      toast.success("Signed out successfully");
    } catch (error) {
      if (error instanceof Error) {
        toast.error(`Failed to sign out: ${error.message}`);
      } else {
        toast.error("Failed to sign out");
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
                item.role === null || hasRole(item.role) ? (
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
                  <span>Username</span>
                  <ChevronUp className="absolute right-2 h-4 w-4" />
                </SidebarMenuButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent side="top" className="w-[var(--radix-popper-anchor-width)]">
                <DropdownMenuItem>
                  <span>Account</span>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <span>Billing</span>
                </DropdownMenuItem>
                <DropdownMenuItem onClick={handleSignOut}>
                  <span>Sign out</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  );
}
