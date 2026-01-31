"use client";

import React, { FC } from "react";
import { cn } from "shared/src/lib/utils";
import IntrinsicElements = JSX.IntrinsicElements;

type HeadingLevel = "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "table" | "title-sm" | "subtitle";

/**
 * Component for rendering headings with consistent styling based on level.
 */
const Heading: FC<{
  level: HeadingLevel,
  children: React.ReactNode,
  className?: string,
  ref?: React.Ref<HTMLHeadingElement>
}> = ({ level = "h1", children, ref, className = "" }) => {
  const Component: keyof IntrinsicElements =
    ({
      h1: "h1",
      h2: "h2",
      h3: "h3",
      h4: "h4",
      h5: "h5",
      h6: "h6",
      table: "h6",
      "title-sm": "h6",
      subtitle: "h5",
    } as const)[level] || "h1";

  const baseClasses = "text-foreground-100 text-balance";
  const sizeClasses =
    {
      h1: "text-2xl mb-4 font-semibold font-title tracking-tight",
      h2: "text-xl mb-3 font-semibold tracking-tight",
      h3: "text-lg mb-3 font-medium tracking-tight",
      h4: "text-base mb-2 font-medium font-display tracking-normal text-white/90",
      h5: "text-sm text-white/80 mb-2 font-medium tracking-wide",
      h6: "text-xs font-semibold mb-1 uppercase tracking-wider text-muted-foreground",
      table: "text-xs font-semibold uppercase tracking-wider",
      "title-sm": "text-sm font-medium mb-1 leading-snug",
      subtitle: "text-base mb-2 font-normal text-muted-foreground max-w-[65ch]",
    }[level] || "";

  return (
    <Component className={cn(baseClasses, sizeClasses, className)}>
      {children}
    </Component>
  );
};

export default Heading;
