import Heading from "shared/src/components/ui/heading";
import { cn } from "shared/src/lib/utils";
import React from "react";

interface PageHeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  title: string;
  badge?: React.ReactNode;
}

export function PageHeader({ title, badge, className, children, ...props }: PageHeaderProps) {
  return (
    <header className={cn("mb-2", className)} {...props}>
       <Heading level="h3" className="flex gap-3 items-center font-display mb-0.5">
        {title}
        {badge}
      </Heading>
      {children}
    </header>
  );
}
