import Heading from "shared/src/components/ui/heading";
import { cn } from "shared/src/lib/utils";
import React from "react";

interface PageSectionProps extends React.HTMLAttributes<HTMLDivElement> {
    title?: string;
    action?: React.ReactNode;
}

/**
 * Section wrapper component that provides consistent spacing and an optional title.
 */
export function PageSection({ title, action, className, children, ...props }: PageSectionProps) {
    return (
        <div className={cn("mb-6", className)} {...props}>
            <div className="flex items-center justify-start gap-4 mb-2">
                {title && <Heading level="h6" className="mt-2 text-white/80 uppercase tracking-wide text-xs font-semibold">{title}</Heading>}
                {action && <div className="mt-1">{action}</div>}
            </div>
            {children}
        </div>
    )
}
