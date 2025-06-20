import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

// @ts-ignore
import { cn } from "@/lib/utils";
// @ts-ignore

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        // primary
        default: "bg-primary text-brand-snow hover:bg-primary/90 hover:text-white shadow",
        destructive: "bg-danger text-danger-50 hover:bg-danger/90 shadow-sm",
        outline:
          "border border-primary-300 text-primary-300 bg-foreground/10 shadow-sm hover:bg-foreground/15 hover:border-primary-400 hover:text-primary-200",
        outline_destructive:
          "border border-danger text-danger-300 bg-foreground/10 shadow-sm hover:bg-foreground/20 hover:text-danger-400",
        secondary:
          "bg-secondary/70 text-secondary-foreground shadow-sm hover:bg-secondary/90",
        ghost: "hover:bg-foreground/10",
        link: "!px-0 text-snow underline-offset-4 hover:underline", // ok
      },
      size: {
        default: "h-9 px-4 py-2",
        sm: "h-8 rounded-md px-3 text-xs",
        lg: "h-10 rounded-md px-8",
        icon: "h-9 w-9",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : "button";
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      />
    );
  }
);
Button.displayName = "Button";

export { Button, buttonVariants };
