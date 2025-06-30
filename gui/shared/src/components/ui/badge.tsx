import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

// @ts-ignore
import {cn} from "@/lib/utils"
// @ts-ignore

import { DragHandleVerticalIcon } from "@radix-ui/react-icons";

const badgeVariants = cva(
  "px-2 py-0.5 w-fit  inline-flex justify-start items-center bg-white/10 font-semibold rounded-sm border whitespace-nowrap shrink-0 gap-1 focus-visible:ring focus-visible:ring-ring/50 focus-visible:ring-[3px] transition-all ",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground border-transparent",
        info: "font-mono uppercase bg-background-800 text-secondary-400 border-transparent",
        role: "text-white uppercase border-transparent",
        status: "bg-opacity-30 border-transparent text-foreground-300", // base status style
        // constraint: "bg-white/10 border-0 border-white/15 text-white/60 font-medium break-all"
      },
      state: {
        /* 
          Importante que los nombres de estas variantes estén en mayúsculas porque es como llega 
          el string del atributo y si no coincide el casing se rompe 
          // para poner en state={process.state} y que pille el nombre directamente
        */
        default: "",
        danger: "",
        warn: "",
        ACTIVE: "bg-process text-process-300 [&>span]:bg-process-400", // Agreements
        ACCEPTED: "bg-process text-process-300 [&>span]:bg-process-400", //
        VERIFIED: "bg-process text-process-300 [&>span]:bg-process-400", // CN
        STARTED: "bg-process text-process-300 [&>span]:bg-process-400", // Transfers
        OFFERED: "bg-warn text-warn-300 [&>span]:bg-warn-400", // CN
        REQUESTED: "bg-warn text-warn-300 [&>span]:bg-warn-400", // CN
        AGREED: "bg-process text-process-300 [&>span]:bg-process-400", //
        FINALIZED: "bg-success text-success-300 [&>span]:bg-success-400", // CN
        COMPLETED: "bg-success text-success-300 [&>span]:bg-success-400", // Transfers
        SUSPENDED: "bg-pause text-pause-300 [&>span]:bg-pause-400", //
        PAUSE: "bg-pause text-pause-300 [&>span]:bg-pause-400", //
        TERMINATED: "bg-danger text-danger-300 [&>span]:bg-danger-400", //
      },
      role: {
        Provider: "bg-roles-provider/30",
        Consumer: "bg-roles-consumer/30",
        Business: "bg-roles-business/30",
        Customer: "bg-roles-customer/30",
      },
      size: {
        default: "text-sm px-2 py-0.5",
        lg: "text-base font-bold bg-white/10 px-2 py-0",
        // cambie paddings, sino eran muy grandes -Clara
        // y le meti bg white sino no se veia nada,
        // estas badges estan en titulos, no sobre el fondo
        //blanco de la tabla, creo que mejor asi.
        sm: "text-xs px-1 py-0.5",
      },
    },
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
  role,
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
      className={cn(badgeVariants({ variant, size, state, role }), className)}
      {...props}
    >
      {/* Mete un circulito si la variable es de tipo status y le asigna el color asignado al estado correspondiente */}
      {showDot && <span className={cn("w-2 h-2 rounded-full mr-1 mb-[2px]")} />}
      {children}
    </Comp>
  );
}

export { Badge, badgeVariants };
