import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import AppSidebar from "@/components/layout/AppSidebar";
import { AppBar } from "./AppBar";
import type { RouteSectionProps } from "@solidjs/router";

export default function Layout(props: RouteSectionProps) {
  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <AppBar />
        <div class="p-3">{props.children}</div>
      </SidebarInset>
    </SidebarProvider>
  );
}
