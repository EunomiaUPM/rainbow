import { QueryClient } from "@tanstack/react-query";
import { createRootRouteWithContext, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { SidebarProvider, SidebarTrigger } from "shared/src/components/ui/sidebar";
import React, { useContext } from "react";
import { Header } from "shared/src/components/header.tsx";
import { AppSidebarBusiness } from "shared/src/components/AppSidebarBusiness";
import { AuthContext, AuthContextType } from "shared/src/context/AuthContext.tsx";

export const Route = createRootRouteWithContext<{
  queryClient: QueryClient;
}>()({
  component: ({ children }: { children: React.ReactNode }) => {
    let widthPage = window.innerWidth;
    const { isAuthenticated } = useContext<AuthContextType | null>(AuthContext)!;
    // console.log("Width:", widthPage);

    return (
      <>
        {isAuthenticated ? (
          <SidebarProvider>
            <div className="fixed flex w-full z-50">
              {isAuthenticated && <AppSidebarBusiness />}
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
        ) : (
          <div>
            <Header />
            <main className="login-container">
              {/*trigger y children se pueden quitar del login?? */}
              {widthPage < 768 ? <SidebarTrigger /> : ""}
              {children}
              <div className="flex h-full w-full ">
                <Outlet />
              </div>
            </main>
          </div>
        )}
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
