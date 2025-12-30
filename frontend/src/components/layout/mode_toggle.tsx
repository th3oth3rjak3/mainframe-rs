import { Monitor, Moon, Sun } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown_menu";
import { useTheme } from "@/components/providers/theme_provider_functions";
import { SidebarGroupAction } from "@/components/ui/sidebar";

export function ModeToggle() {
  const { theme, setTheme } = useTheme();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <SidebarGroupAction title="Toggle Theme" className="p-2 w-8 rounded-full">
          {/* Sun Icon: Visible in light mode */}
          <Sun
            className={`h-4 w-4 transition-all ${
              theme === "light" ? "rotate-0 scale-100" : "-rotate-90 scale-0"
            }`}
          />
          {/* Moon Icon: Visible in dark mode */}
          <Moon
            className={`absolute h-4 w-4 transition-all ${
              theme === "dark" ? "rotate-0 scale-100" : "rotate-90 scale-0"
            }`}
          />
          {/* Monitor Icon: Visible in system mode */}
          <Monitor
            className={`absolute h-4 w-4 transition-all ${
              theme === "system" ? "rotate-0 scale-100" : "rotate-90 scale-0"
            }`}
          />
        </SidebarGroupAction>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem
          onClick={() => setTheme("light")}
          className={theme === "light" ? "bg-accent" : ""}
        >
          <Sun /> Light
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => setTheme("dark")}
          className={theme === "dark" ? "bg-accent" : ""}
        >
          <Moon /> Dark
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => setTheme("system")}
          className={theme === "system" ? "bg-accent" : ""}
        >
          <Monitor /> System
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
