import Heading from "shared/src/components/ui/heading";
import { cn } from "shared/src/lib/utils";
import React from "react";

interface PageHeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  title: string;
  badge?: React.ReactNode;
}

/**
 * Standard header component for pages, displaying a title and optional badge.
 */
export function PageHeader({ title, badge, className, children, ...props }: PageHeaderProps) {
  return (
    <header className={cn("mb-1 pb-1 border-b border-white/5", className)} {...props}>
      <Heading level="h4" className="flex gap-2 items-center font-display mb-0 text-white/90">
        {title}
        {badge}
      </Heading>
      {children}
    </header>
  );
}
