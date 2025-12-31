import * as React from "react";
import { cn } from "@/lib/utils";
import { Input, type InputProps } from "@/shared/ui/input";
import { Button } from "@/shared/ui/button";
import { X } from "lucide-react";

export interface ClearableInputProps extends InputProps {
  onClear: () => void;
}

const ClearableInput = React.forwardRef<HTMLInputElement, ClearableInputProps>(
  ({ className, value, onClear, ...props }, ref) => {
    const hasValue = value && String(value).length > 0;

    return (
      <div className="relative">
        <Input ref={ref} value={value} className={cn("pr-10", className)} {...props} />
        {hasValue && (
          <Button
            type="button"
            variant="ghost"
            size="icon"
            className="absolute right-1 top-1/2 h-6 w-6 -translate-y-1/2"
            onClick={onClear}
          >
            <X className="h-4 w-4 text-muted-foreground" />
            <span className="sr-only">Clear</span>
          </Button>
        )}
      </div>
    );
  }
);

ClearableInput.displayName = "ClearableInput";

export { ClearableInput };
