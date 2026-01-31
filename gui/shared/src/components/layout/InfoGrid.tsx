import { cn } from "shared/src/lib/utils";
import React from "react";

interface InfoGridProps extends React.HTMLAttributes<HTMLDivElement> { }

/**
 * Grid layout wrapper for displaying lists of key-value information.
 */
export function InfoGrid({ className, children, ...props }: InfoGridProps) {
  return (
    <div className={cn("grid grid-cols-1 md:grid-cols-2 gap-3 mb-4", className)} {...props}>
      {children}
    </div>
  );
}
