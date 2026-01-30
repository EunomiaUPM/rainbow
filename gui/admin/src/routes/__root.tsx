import { QueryClient } from "@tanstack/react-query";
import { createRootRouteWithContext, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { SidebarProvider, SidebarTrigger } from "../../../shared/src/components/ui/sidebar";
import React from "react";
import { AppSidebarProvider } from "shared/src/components/AppSidebarProvider.tsx";
import { Header } from "shared/src/components/header.tsx";

export const Route = createRootRouteWithContext<{
  queryClient: QueryClient;
}>()({
  component: ({ children }: { children: React.ReactNode }) => {
    const widthPage = window.innerWidth;
    // console.log("Width:", widthPage);
    return (
      <>
        <SidebarProvider>
          <div className="fixed flex w-full z-50">
            <AppSidebarProvider />
            <Header />
          </div>
          <main className="page-container">
            {/* boton de abrir/cerrar sidebar solo para movil/pantalla pequeña */}
            {widthPage < 768 ? <SidebarTrigger /> : ""}
            {children}
            <div className="main-container">
              <Outlet />
            </div>
          </main>
        </SidebarProvider>
        {/* <div className="p-2 flex gap-2">
                <Link to="/" className="[&.active]:font-bold text-foreground text-decoration-none">
                    Home for provider
                </Link>{" "}
            </div> */}
        <hr />
        <TanStackRouterDevtools />
      </>
    );
  },
});
