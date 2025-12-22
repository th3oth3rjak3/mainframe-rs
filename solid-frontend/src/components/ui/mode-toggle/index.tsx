import { Monitor, Moon, Sun } from "lucide-solid";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { SidebarGroupAction } from "@/components/ui/sidebar";
import { localStorageManager, useColorMode, type ConfigColorMode } from "@kobalte/core";
import { createSignal, Match, Switch } from "solid-js";

export function ModeToggle() {
  const { setColorMode } = useColorMode();
  const [theme, setTheme] = createSignal<ConfigColorMode>("system");

  const handleOpenChange = (open: boolean) => {
    if (open) {
      // Read from localStorage when dropdown opens
      const stored = localStorageManager.get();
      if (stored) {
        setTheme(stored);
      }
    }
  };

  const handleThemeChange = (value: ConfigColorMode) => {
    setTheme(value);
    setColorMode(value);
  };

  return (
    <DropdownMenu onOpenChange={handleOpenChange}>
      <DropdownMenuTrigger
        as={SidebarGroupAction}
        title="Toggle Theme"
        class="p-2 w-8 rounded-full"
      >
        <Switch>
          <Match when={theme() === "light"}>
            <Sun />
          </Match>
          <Match when={theme() === "dark"}>
            <Moon />
          </Match>
          <Match when={theme() === "system"}>
            <Monitor />
          </Match>
        </Switch>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        <DropdownMenuItem
          onClick={() => handleThemeChange("light")}
          class={theme() === "light" ? "bg-accent" : ""}
        >
          <Sun /> Light
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => handleThemeChange("dark")}
          class={theme() === "dark" ? "bg-accent" : ""}
        >
          <Moon /> Dark
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => handleThemeChange("system")}
          class={theme() === "system" ? "bg-accent" : ""}
        >
          <Monitor /> System
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
