import { cn } from "shared/src/lib/utils";
import React from "react";

interface PageLayoutProps extends React.HTMLAttributes<HTMLDivElement> { }

/**
 * Main container component that provides consistent padding and spacing for pages.
 */
export function PageLayout({ className, children, ...props }: PageLayoutProps) {
  return (
    <div className={cn("space-y-2 pb-2 px-3 w-full", className)} {...props}>
      {children}
    </div>
  );
}
