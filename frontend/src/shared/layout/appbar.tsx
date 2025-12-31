import { ChevronRight } from "lucide-react";
import { SidebarTrigger } from "@/shared/ui/sidebar";
import { useLocation, useNavigate } from "@tanstack/react-router";

export function AppBar() {
  const location = useLocation();
  const navigate = useNavigate();

  // Generate breadcrumbs from the current path
  const generateBreadcrumbs = () => {
    const paths = location.pathname.split("/").filter(Boolean);

    const breadcrumbs = [{ label: "Home", path: "/" }];

    let buildPath = "";
    paths.forEach((segment) => {
      buildPath += `/${segment}`;

      // Auto-capitalize and format the segment
      const label = segment.charAt(0).toUpperCase() + segment.slice(1).replace(/-/g, " ");

      breadcrumbs.push({
        label,
        path: buildPath,
      });
    });

    return breadcrumbs;
  };

  const breadcrumbs = generateBreadcrumbs();

  const handleNavigation = (path: string) => {
    navigate({ to: path });
  };

  return (
    <div className="h-14 border-b bg-background px-4 flex items-center justify-between gap-4">
      {/* Left side with trigger and breadcrumbs */}
      <div className="flex items-center gap-4">
        <SidebarTrigger />
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          {breadcrumbs.map((crumb, index) => (
            <div key={crumb.path} className="flex items-center gap-2">
              {index === breadcrumbs.length - 1 ? (
                // Last breadcrumb (current page) - not clickable
                <span className="text-foreground font-medium">{crumb.label}</span>
              ) : (
                // Previous breadcrumbs - clickable links
                <>
                  <button
                    onClick={() => handleNavigation(crumb.path)}
                    className="hover:text-foreground transition-colors cursor-pointer"
                  >
                    {crumb.label}
                  </button>
                  <ChevronRight className="h-4 w-4" />
                </>
              )}
            </div>
          ))}
        </div>
      </div>
      {/* Right side actions */}
      <div className="flex items-center gap-2">{/* Add search or other icons here later */}</div>
    </div>
  );
}
