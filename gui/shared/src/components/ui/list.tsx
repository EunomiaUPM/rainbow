import * as React from "react";
import { cn } from "./../../lib/utils";

const List = ({ className, type, ...props }, ref) => {

    return (
        <ul
            ref={ref}
            className={cn(
                      " w-[500px] text-sm flex justify-center relative  bg-white/5 overflow-auto border border-[#ff0000]/60 rounded-md",
                      className,
                    )}
            {...props}
        >
            {props.children}
        </ul>
    )
 }

 export { List };