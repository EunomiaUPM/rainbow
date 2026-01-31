import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "shared/src/lib/utils";

/**
 * Displays a badge or a component that looks like a badge.
 */
const badgeVariants = cva(
  "px-2 py-0.5 w-fit  inline-flex justify-start items-center bg-white/10 font-semibold rounded-sm border whitespace-nowrap shrink-0 gap-1 focus-visible:ring focus-visible:ring-ring/50 focus-visible:ring-[3px] transition-all ",
  {
    variants: {
      variant: {
        default: "bg-brand-snow/15 text-brand-snow border-transparent",
        info: "font-mono uppercase bg-background-800 text-secondary-400 border-transparent",
        infoLighter: "font-mono uppercase bg-white/10 text-secondary-400 border-transparent",
        role: "text-white uppercase border-transparent",
        status: "bg-opacity-30 border-transparent text-foreground-300",
      },
      state: {
        default: "",
        danger: "",
        warn: "",
        ACTIVE: "bg-process text-process-300 [&>span]:bg-process-400",
        INACTIVE: "bg-paused text-paused-300 [&>span]:bg-paused-400",
        ACCEPTED: "bg-process text-process-300 [&>span]:bg-process-400",
        VERIFIED: "bg-process text-process-300 [&>span]:bg-process-400",
        STARTED: "bg-process text-process-300 [&>span]:bg-process-400",
        OFFERED: "bg-warn text-warn-300 [&>span]:bg-warn-400",
        REQUESTED: "bg-warn text-warn-300 [&>span]:bg-warn-400",
        AGREED: "bg-process text-process-300 [&>span]:bg-process-400",
        FINALIZED: "bg-success text-success-300 [&>span]:bg-success-400",
        COMPLETED: "bg-success text-success-300 [&>span]:bg-success-400",
        SUSPENDED: "bg-pause text-pause-300 [&>span]:bg-pause-400",
        PAUSE: "bg-pause text-pause-300 [&>span]:bg-pause-400",
        BY_PROVIDER: "bg-pause text-pause-300 [&>span]:bg-pause-400",
        BY_CONSUMER: "bg-pause text-pause-300 [&>span]:bg-pause-400",
        ON_REQUEST: "bg-pause text-pause-300 [&>span]:bg-pause-400",
        STOP: "bg-pause text-pause-300 [&>span]:bg-pause-400",
        STOPPED: "bg-pause text-pause-300 [&>span]:bg-pause-400",
        TERMINATED: "bg-danger text-danger-300 [&>span]:bg-danger-400",
      },
      dsrole: {
        Provider: "bg-roles-provider/30",
        Consumer: "bg-roles-consumer/30",
        Business: "bg-roles-business/30",
        Customer: "bg-roles-customer/30",
      },
      size: {
        default: "text-sm px-2 py-0.5",
        lg: "text-lg font-bold px-2 py-0",
        sm: "text-xs px-1 py-0.5",
      },
    },
    defaultVariants: {
      variant: "default",
      state: "default",
      size: "default",
    },
  },
);

export type BadgeState = VariantProps<typeof badgeVariants>["state"];
export type BadgeRole = VariantProps<typeof badgeVariants>["dsrole"];

function Badge({
  className,
  variant,
  state,
  size,
  dsrole,
  asChild = false,
  children,
  ...props
}: React.ComponentProps<"span"> & VariantProps<typeof badgeVariants> & {
  asChild?: boolean
}) {
  const Comp = asChild ? Slot : "span";
  const showDot = variant === "status";

  return (
    <Comp
      data-slot="badge"
      className={cn(badgeVariants({ variant, size, state, dsrole }), className)}
      {...props}
    >
      {showDot && <span className={cn("w-2 h-2 rounded-full mr-1 mb-[2px]")} />}
      {children}
    </Comp>
  );
}

export { Badge, badgeVariants };
