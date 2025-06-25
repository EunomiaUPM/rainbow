import * as React from "react";
import { cn } from "./../../lib/utils";

const List = ({
  className,
  children,
  ...props
}: {
  className?: string;
  children: React.ReactNode;
  props?: any;
}) => {
  return (
    <ul
      className={cn(
        " !mt-0 px-2 xl:w-[450px] 2xl:w-[500px] text-sm flex flex-col justify-center relative  bg-brand-sky/5 overflow-auto border border-white/10 rounded-md",
        className
      )}
      {...props}
    >
      {children}
    </ul>
  );
};

const ListItem = ({
  className,
  children,
  ...props
}: {
  className?: string;
  children: React.ReactNode;
  props?: any;
}) => {
  return (
    <li
      className={cn(
        "h-fit min-h-9 max-w-full py-1.5 flex flex-row tracking-wide flex-wrap justify-start items-center gap-2 border-b border-white/20 last:border-0", // place-content-center 
        className
      )}
      {...props}
    >
      {children}
    </li>
  );
};

const ListItemKey = ({
  className,
  children,
  ...props
}: {
  className?: string;
  children: React.ReactNode;
  props?: any;
}) => {
  return (
    <p className={cn("font-bold basis-[45%] min-w-fit", className)} {...props}>
      {children}
    </p>
  );
};

const ListItemDate = ({
  className,
  children,
  ...props
}: {
  className?: string;
  children: React.ReactNode;
  props?: any;
}) => {
  return (
    <p className={cn("text-gray-400 ", className)} {...props}>
      {children}
    </p>
  );
};

export { List, ListItem, ListItemKey, ListItemDate };
