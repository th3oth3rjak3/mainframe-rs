// src/components/ui/page_header.tsx
import * as React from "react";
import { cn } from "@/lib/utils"; // Assuming you use shadcn/ui's utility function

interface PageHeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  title: string;
  description?: string;
  actions?: React.ReactNode;
}

export function PageHeader({ title, description, actions, className, ...props }: PageHeaderProps) {
  return (
    <div className={cn("flex items-center justify-between pb-6", className)} {...props}>
      {/* Title and Description on the left */}
      <div>
        <h1 className="text-2xl font-semibold tracking-tight">{title}</h1>
        {description && <p className="text-muted-foreground mt-1">{description}</p>}
      </div>

      {/* Actions on the right */}
      {actions && <div>{actions}</div>}
    </div>
  );
}
