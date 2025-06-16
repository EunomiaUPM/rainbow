"use client";

import React from "react";

const Heading = ({ level = "h1", children, className = "" }) => {
  const Component = {
    h1: "h1", h2: "h2", h3: "h3", h4: "h4", 
    h5: "h5", h6: "h6",
    "table": "h6", "title-sm": "h6", subtitle: "h5",
  }[level] || "h1";

  const baseClasses = "text-foreground-100 text-balance";
  const sizeClasses = {
    h1: "md:text-36 sm:text-40 mb-6 font-medium font-title",
    h2: "md:text-32 sm:text-36 mb-4 font-medium",
    h3: "md:text-28 sm:text-32 mb-4 font-medium",
    h4: "md:text-24 sm:text-28 mb-2 font-medium font-display", // lo de poner la clase de la tipo no funciona aqui
    //por eso lo he metido en el index.css
    h5: "md:text-20 sm:text-24 mb-2 font-medium",
    h6: "text-base font-medium  mb-2",
    "table": "text-base font-medium ",
    "title-sm": "text-base sm:text-20 font-normal mb-2 leading-snug",
    subtitle: "text-20 md:text-24 mb-2 font-light max-w-[50ch] md:max-w-[70ch] 2xl:max-w-[75ch]",
  }[level] || "";

  return (
    <Component className={baseClasses + " " + sizeClasses + " " + className}>
      {children}
    </Component>
  );
};

export default Heading;
