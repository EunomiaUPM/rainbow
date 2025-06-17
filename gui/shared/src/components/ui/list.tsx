import * as React from "react";
import { cn } from "./../../lib/utils";

const List = ({ className, ...props }) => {
    return (
        <ul 
            className={cn(
                      " !mt-0 px-2 w-[500px] text-sm flex flex-col justify-center relative  bg-white/5 overflow-auto border border-white/10 rounded-md",
                      className,
                    )}
            {...props}
        >
            {props.children}
        </ul>
    )
 }

 const ListItem = ({ className, ...props }) => {
    return (
          <li 
          className="h-9 flex flex-row justify-start gap-4 border-b border-white/20 last:border-0 items-center"
           {...props}>
           {props.children}
          </li>
    )
 }

  const ListItemKey = ({ className, ...props }) => {
    return (
          <p 
          className={cn("font-bold w-1/2",className )}
          {...props}>
             {props.children}
          </p>
    )
 }

 export { List, ListItem, ListItemKey };