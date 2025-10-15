import * as React from "react";
import {Slot} from "@radix-ui/react-slot";
import {cva, type VariantProps} from "class-variance-authority";
import {cn} from "shared/src/lib/utils";


const buttonVariants = cva(
  "inline-flex items-center flex-nowrap min-w-fit tracking-wider !leading-none justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium uppercase transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        // primary
        default: "bg-primary text-brand-snow hover:bg-primary/90 hover:text-white shadow",
        // destructive: "bg-danger text-danger-50 hover:bg-danger/90 shadow-sm",
        destructive:
          "border border-danger text-danger-300 bg-foreground/10 shadow-sm hover:bg-foreground/20 hover:text-danger-400",
        outline:
          "border border-primary-300 text-primary-300 bg-foreground/10 shadow-sm hover:bg-foreground/15 hover:border-primary-400 hover:text-primary-200",
        outline_destructive:
          "border border-danger text-danger-300 bg-foreground/10 shadow-sm hover:bg-foreground/20 hover:text-danger-400",
        secondary: "bg-secondary/70 text-secondary-foreground shadow-sm hover:bg-secondary/90",
        ghost: "text-brand-snow bg-foreground/10 hover:bg-foreground/20",
        icon_destructive:
          "flex text-danger-400 border border-danger hover:text-danger-500 bg-foreground/5 hover:bg-foreground/10 p-0 mb-0 [&_svg]:w-5 [&_svg]:h-5 p-1",
        link: "!px-0 flex-no-wrap normal-case text-snow underline-offset-4 hover:underline", // ok
      },
      size: {
        default: "h-9 px-4 py-2",
        sm: "h-8 rounded-md px-3 text-xs",
        xs: "h-7 rounded-md px-3 text-xs",
        lg: "h-10 rounded-md px-8 text-lg",
        icon: "h-9 w-9 rounded-full p-1",
        icon_sm: "h-7 w-7 rounded-full p-0",
      },
      policy: {
        default: "",
        permission: "bg-success-600/20 border-success-700 text-success-100 hover:bg-success-600/30",
        obligation: "bg-warn-700/20 border-warn-700 text-warn-100 hover:bg-warn-700/30",
        prohibition: "bg-danger-600/20 border-danger-700 text-danger-100 hover:bg-danger-600/30",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
      policy: "default",
    },
  },
);

export type ButtonSizes = VariantProps<typeof buttonVariants>["size"];

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({className, variant, size, policy, asChild = false, ...props}, ref) => {
    const Comp = asChild ? Slot : "button";
    return (
      <Comp
        className={cn(buttonVariants({variant, size, policy, className}))}
        ref={ref}
        {...props}
      />
    );
  },
);
Button.displayName = "Button";

export {Button, buttonVariants};
