import * as React from "react";
import { useRouterState } from "@tanstack/react-router";
import SearchIcon from "@mui/icons-material/Search";
// @ts-ignore
import { cn } from "@/lib/utils";
// @ts-ignore

const Input = React.forwardRef<HTMLInputElement, React.ComponentProps<"input">>(
  ({ className, type, ...props }, ref) => {
    // console.log(type, "type")
    const routerState = useRouterState();
    let pathsArray = routerState.location.pathname.split("/");

    // Quitar del pathArray los elementos que contienen un IdleDeadline,
    // que llevan las letras "urn" delante para que no salgan
    // en el input placeholder
    pathsArray.map((path, index) => {
      path.includes("urn") ? pathsArray.splice(index) : "";
     
      // console.log(pathsArray, "pathsArray");
      return pathsArray;
    });
    pathsArray.splice(0, 1); // Eliminar el primer elemento vacío
   let stringPathsArray = JSON.stringify(pathsArray);
   console.log(stringPathsArray, "stringPathsArray");
    // console.log(pathsArray, "pathsArray2");
    let pathFormat = stringPathsArray
      .replace(/["[\]]/g, "")
      .split("-")
      .join(" ");
    console.log(pathFormat, "pathFormatete");
    let placeHolderText;

    // SI el boton es de busqueda, texto de busqueda + ruta
    // en la que se encuentra el usuario
    // Si no, texto de entrada normal
    if (type === "search") {
      console.log(pathFormat, "pathFormatto");
      placeHolderText = "Search for " + pathFormat;
      // en la pagina de catálogo lo que se busca es dataset
    } else {
      placeHolderText = "Enter text";
    }

    return (
      <div
        className={cn(
          "flex h-9 items-center rounded-md border-0 border-input bg-brand-snow/5 pl-3 text-sm focus-within:ring-1 focus-within:ring-ring ",
          className
        )}
      >
        {type === "search" && <SearchIcon />}
        <input
          {...props}
          type="search"
          ref={ref}
          placeholder={placeHolderText}
          className="bg-transparent w-full p-2 placeholder:text-muted-foreground focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        />
      </div>
    );
  }
);
Input.displayName = "Input";

export { Input };
