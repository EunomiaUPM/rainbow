import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";
import { DragHandleVerticalIcon } from "@radix-ui/react-icons";

const badgeVariants = cva(
  "inline-flex items-center justify-center font-semibold rounded-sm border px-2 py-0.5 w-fit whitespace-nowrap shrink-0 gap-1 focus-visible:ring focus-visible:ring-ring/50 focus-visible:ring-[3px] transition-all overflow-hidden",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground border-transparent",
        info: "font-mono uppercase bg-background-800 text-secondary-400 border-transparent",
        role: "text-white uppercase bg-foreground-950 border-transparent",
        status: "bg-opacity-30 border-transparent text-foreground-300", // base status style
      },
      state: {
        default: "",
        danger: "",
        warn: "",
        finalized: "", // les he metido los nombres que habían ya,
        // para poner en state={process.state} y que pille el nombre directamente, 
        completed: "",
        started: "",
        pause: "",
      },
      size: {
        default: "text-sm px-2 py-0.5",
        lg: "text-base font-bold bg-white/10 px-2 py-0.5", 
        // cambie paddings, sino eran muy grandes -Clara
        // y le meti bg white sino no se veia nada,
        // estas badges estan en titulos, no sobre el fondo
        //blanco de la tabla, creo que mejor asi.
        sm: "text-xs px-1 py-0.5",
      },
    },
    compoundVariants: [ 

      {
        variant: "status",
        state: "danger",
        class: "bg-danger text-danger-300 [&>span]:bg-danger-400",
      },
      {
        variant: "status",
        state: "warn",
        class: "bg-warn text-warn-300 [&>span]:bg-warn-400",
      },
      {
        variant: "status",
        state: "finalized",
        class: "bg-success text-success-300 [&>span]:bg-success-400",
      },
      // estilo de completed = finalized
       {
        variant: "status",
        state: "completed",
        class: "bg-success text-success-300 [&>span]:bg-success-400",
      },
      {
        variant: "status",
        state: "started",
        class: "bg-process text-process-300 [&>span]:bg-process-400",
      },
      {
        variant: "status",
        state: "pause",
        class: "bg-pause text-pause-300 [&>span]:bg-pause-400",
      },
    ],
    defaultVariants: {
      variant: "default",
      state: "default",
      size: "default",
    },
  }
);

function Badge({
  className,
  variant,
  state,
  size,
  asChild = false,
  children,
  ...props
}: React.ComponentProps<"span"> &
  VariantProps<typeof badgeVariants> & { asChild?: boolean }) {
  const Comp = asChild ? Slot : "span";
  const showDot = variant === "status";

  return (
    <Comp
      data-slot="badge"
      className={cn(badgeVariants({ variant, size, state }), className)}
      {...props}
    >
      {/* Mete un circulito si la variable es de tipo status y le asigna el color asignado al estado correspondiente */}
      {showDot && (
        <span className={cn("w-2 h-2 rounded-full mr-1")} />
      )}
      {children}
    </Comp>
  );
}

export { Badge, badgeVariants };
