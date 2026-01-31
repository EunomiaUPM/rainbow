import { cn } from "shared/src/lib/utils";
import React from "react";

interface InfoGridProps extends React.HTMLAttributes<HTMLDivElement> { }

/**
 * Grid layout wrapper for displaying lists of key-value information.
 */
export function InfoGrid({ className, children, ...props }: InfoGridProps) {
  return (
    <div className={cn("gridColsLayout", className)} {...props}>
      {children}
    </div>
  );
}
