import * as React from "react"
import {
  useRouterState,
} from "@tanstack/react-router";
import SearchIcon from '@mui/icons-material/Search';;
import { cn } from "@/lib/utils"


const Input = React.forwardRef<HTMLInputElement, React.ComponentProps<"input">>(
  
  ({ className, type, ...props }, ref) => {
    // console.log(type, "type")
      const routerState = useRouterState();
      let pathFormat = routerState.location.pathname.split("/").slice(1).join("").split("-").join(" ");
      let placeHolderText;

      // SI el boton es de busqueda, texto de busqueda + ruta
      // en la que se encuentra el usuario
      // Si no, texto de entrada normal
      if (type === "search") { 
        placeHolderText = "Search for " + pathFormat;
      } else {
        placeHolderText= "Enter text"
      }
       
      return (
         <div
        className={cn(
          "flex h-9 items-center rounded-md border border-0 bg-white/10 pl-3 text-sm  focus-within:ring-1 focus-within:ring-ring ",
          className,
        )}
      >
   {type === "search" && <SearchIcon className="mt-0.5" />}
        <input
          {...props}
          type="search"
          ref={ref}
          placeholder={placeHolderText}
          className="bg-transparent w-full p-2 placeholder:text-muted-foreground focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        />
      </div>
    )
  }
)
Input.displayName = "Input"

export { Input }
