import * as React from "react";
import * as SeparatorPrimitive from "@radix-ui/react-separator";

// @ts-ignore
import { cn } from "@/lib/utils";
// @ts-ignore

const Separator = React.forwardRef<
  React.ElementRef<typeof SeparatorPrimitive.Root>,
  React.ComponentPropsWithoutRef<typeof SeparatorPrimitive.Root>
>(({ className, orientation = "horizontal", decorative = true, ...props }, ref) => (
  <SeparatorPrimitive.Root
    ref={ref}
    decorative={decorative}
    orientation={orientation}
    className={cn(
      "shrink-0 bg-stroke",
      orientation === "horizontal" && "!h-[1px] w-full",
      orientation === "vertical" && "h-full !w-[1px]",
      className,
    )}
    {...props}
  />
));
Separator.displayName = SeparatorPrimitive.Root.displayName;

export { Separator };
