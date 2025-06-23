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
        " !mt-0 px-2 w-[500px] text-sm flex flex-col justify-center relative  bg-brand-sky/5 overflow-auto border border-white/10 rounded-md",
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
        "h-fit py-1.5 place-content-center flex flex-row justify-start items-start gap-4 border-b border-white/20 last:border-0 ",
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
    <p className={cn("font-bold w-1/2  ", className)} {...props}>
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
