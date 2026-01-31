import { cn } from "shared/src/lib/utils";
import React from "react";

interface InfoGridProps extends React.HTMLAttributes<HTMLDivElement> {}

export function InfoGrid({ className, children, ...props }: InfoGridProps) {
  return (
    <div className={cn("gridColsLayout", className)} {...props}>
      {children}
    </div>
  );
}
