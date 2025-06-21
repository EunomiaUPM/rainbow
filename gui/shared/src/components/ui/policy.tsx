import * as React from "react";
import { cn } from "./../../lib/utils";
import { cva, type VariantProps } from "class-variance-authority";
import { Badge } from "./badge"

const policyVariants = cva("gap-3 border px-3 py-2 rounded-md", {
  variants: {
    variant: {
      permission: "bg-success-600/20 border-success-700 text-sucess-100",
      obligation: "bg-warn-700/20 border-warn-700 text-warn-100",
      prohibition: "bg-danger-600/20 border-danger-700 text-danger-100",
    },
  },
});


export interface PolicyProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof policyVariants> {
  children: React.ReactNode;
}

const Policy = React.forwardRef<HTMLDivElement, PolicyProps>(
  ({ className, variant, children, ...props }, ref) => {
    return (
      <div className={cn(policyVariants({ variant, className }))} {...props}>
        {children}
      </div>
    );
  }
);
Policy.displayName = "Policy";
const PolicyItemContainer = ({  children, ...props}) => {
return (
        <div className={cn("flex flex-col gap-1" )}  {...props}>
                {children}
        </div>
    )
}
const PolicyItem = ({  children, ...props}) => {
return (
        <div className={cn("flex" )}  {...props}>
                {children}
        </div>
    )
}

const PolicyItemKey = ({ children, ...props}) => {
    return (
        <div className={cn("w-32 font-semibold" )}  {...props}>
          {children}
        </div>
    )
}


const PolicyItemValue = ({ children, ...props}) => {
    return (
        <div className={cn("uppercase" )}  {...props}>
          {children}
        </div>
    )
}



const PolicyConstraintsContainer = ({  children, ...props}) => {
    //  const formatString = (text: string) => {
    //     let formattedText = text.replace(/[()\[\]{}"]/g, ' ')
    //     return formattedText
    // }
    return (
        <div className={cn("flex gap-2")}  {...props}>
                {children}
        </div>
    )
}

const PolicyConstraint = ({ type, className, children, ...props}) => {
      const formatString = (text: string) => {
        let formattedText = text.replace(/[()\[\]{},"]/g, ' ')
        return formattedText
    }
    // VERIFICAR SI EL TEXTO QUE VIENE ES TIPO STRING
    // SI NO LO ES, PASARLO A STRING PARA PODER FORMATEARLO
      const childText = typeof children === 'string' ? children : String(children)
    return (
        <div className={className} {...props}>
               <Badge variant="constraint"> {formatString(childText)}
                </Badge>
               <div className="text-2xs opacity-60 mt-1">{type}</div>
        </div>
    )
}

export { Policy, policyVariants,PolicyItemContainer, PolicyItem, PolicyItemKey,PolicyItemValue, PolicyConstraint, PolicyConstraintsContainer };
