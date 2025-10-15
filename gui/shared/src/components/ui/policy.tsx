import * as React from "react";
import {FC} from "react";
import {cn} from "shared/src/lib/utils";
import {cva, type VariantProps} from "class-variance-authority";

const policyVariants = cva("gap-1.5 border px-2 py-1 rounded-md", {
  variants: {
    variant: {
      permission: "bg-success-600/20 border-success-700 text-sucess-100",
      obligation: "bg-warn-700/20 border-warn-700 text-warn-100",
      prohibition: "bg-danger-600/20 border-danger-700 text-danger-100",
    },
  },
});

export type PolicyVariants = VariantProps<typeof policyVariants>["variant"];

const HeadingColor = ({variant}: { variant: PolicyVariants }) => {
  switch (variant) {
    case "permission":
      return "text-success-200";
    case "obligation":
      return "text-warn-300";
    case "prohibition":
      return "text-danger-400";
    default:
      return "text-white/80"; // Default color if no variant matches
  }
};

export interface PolicyProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof policyVariants> {
  children: React.ReactNode;
}

const Policy = React.forwardRef<HTMLDivElement, PolicyProps>(
  ({className, variant, children, ...props}, ref) => {
    return (
      <div className={cn(policyVariants({variant, className}))} {...props}>
        <p className={`uppercase mb-0 font-bold ${HeadingColor({variant})}`}>{variant}</p>
        {children}
      </div>
    );
  },
);
Policy.displayName = "Policy";
const PolicyItemContainer: FC<{
  children: React.ReactNode;
  props?: React.HTMLAttributes<HTMLDivElement>;
}> = ({children, ...props}) => {
  return (
    <div
      className={cn(
        "flex flex-col gap-1 py-3 border-b border-white/20 last:border-0 first:pt-1 last:pb-1",
      )}
      {...props}
    >
      {children}
    </div>
  );
};
const PolicyHeading: FC<{
  children: React.ReactNode;
  props?: React.HTMLAttributes<HTMLDivElement>;
}> = ({children, ...props}) => {
  return (
    <div className={cn("flex ")} {...props}>
      {children}
    </div>
  );
};

const PolicyItem: FC<{
  children: React.ReactNode;
  props?: React.HTMLAttributes<HTMLDivElement>;
}> = ({children, ...props}) => {
  return (
    <div className={cn("flex ")} {...props}>
      {children}
    </div>
  );
};

const PolicyItemKey: FC<{
  children: React.ReactNode;
  props?: React.HTMLAttributes<HTMLDivElement>;
}> = ({children, ...props}) => {
  return (
    <div className={cn("w-32 font-semibold text-white/60")} {...props}>
      {children}
    </div>
  );
};

const PolicyItemValue: FC<{
  children: React.ReactNode;
  props?: React.HTMLAttributes<HTMLDivElement>;
}> = ({children, ...props}) => {
  return (
    <div className={cn("uppercase")} {...props}>
      {children}
    </div>
  );
};

const PolicyConstraintsContainer: FC<{
  children: React.ReactNode;
  props?: React.HTMLAttributes<HTMLDivElement>;
}> = ({children, ...props}) => {
  return (
    <div className={cn("flex gap-1.5 bg-black/30 w-fit p-1 rounded-md")} {...props}>
      {children}
    </div>
  );
};

const PolicyConstraint: FC<{
  children: React.ReactNode;
  type: "leftOperand" | "operator" | "rightOperand";
  className?: string;
  props?: React.HTMLAttributes<HTMLDivElement>;
}> = ({type, className, children, ...props}) => {
  const formatString = (text: string) => {
    let formattedText = text.replace(/[()\[\]{},"]/g, " ");
    return formattedText;
  };
  const childText = typeof children === "string" ? children : String(children);
  return (
    <div className={cn("constraint-policy-container", className)} {...props}>
      <span
        className={`flex justify-start items-start h-full px-2 py-0.5 w-fit max-w-[165px] rounded-sm gap-1 focus-visible:ring focus-visible:ring-ring/50 focus-visible:ring-[3px] transition-all   font-medium break-all border border-white/15 bg-gray-300/5
    ${childText.length >= 16 ? "nowrap" : ""}`}
      >
        <p className="break-all text-white/80">{formatString(childText)}</p>
        {/* {console.log(childText.length, " childText")} */}
      </span>
      <div
        className="constraint-item text-2xs px-1.5 rounded-sm py-0.5 cursor-pointer bg-black/90 opacity-80 mt-1">
        {type}
      </div>
    </div>
  );
};

const PolicyConstraintsWrapper: FC<{
  children: React.ReactNode;
  props?: React.HTMLAttributes<HTMLDivElement>;
}> = ({children}) => {
  return <div className="flex flex-col gap-1">{children}</div>;
};


export {
  Policy,
  policyVariants,
  PolicyItemContainer,
  PolicyHeading,
  PolicyItem,
  PolicyItemKey,
  PolicyItemValue,
  PolicyConstraint,
  PolicyConstraintsContainer,
  PolicyConstraintsWrapper,
};
