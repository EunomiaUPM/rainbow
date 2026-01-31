import Heading from "shared/src/components/ui/heading";
import { cn } from "shared/src/lib/utils";
import React from "react";

interface PageSectionProps extends React.HTMLAttributes<HTMLDivElement> {
    title?: string;
}

/**
 * Section wrapper component that provides consistent spacing and an optional title.
 */
export function PageSection({ title, className, children, ...props }: PageSectionProps) {
    return (
        <div className={cn("mb-4", className)} {...props}>
            {title && <Heading level="h5" className="mb-2 mt-3">{title}</Heading>}
            {children}
        </div>
    )
}
