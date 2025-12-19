import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";

import AppSidebar from "@/components/layout/app-sidebar";
import { AppBar } from "@/components/layout/appbar";
import { Outlet } from "react-router-dom";

export default function Layout() {
  return (
    <SidebarProvider>
      <AppSidebar variant="floating" />
      <SidebarInset>
        <AppBar />
        <main className="p-4">
          <Outlet />
        </main>
      </SidebarInset>
    </SidebarProvider>
  );
}
