import { QueryClient } from "@tanstack/react-query";
import {
  createRootRouteWithContext,
  Link,
  Outlet,
} from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import {
  SidebarProvider,
  SidebarTrigger,
} from "../../../shared/src/components/ui/sidebar";
import React from "react";
import { AppSidebar } from "../../../shared/src/components/app-sidebar.tsx";
import { Header } from "shared/src/components/header.tsx";

export const Route = createRootRouteWithContext<{
  queryClient: QueryClient;
}>()({
  component: ({ children }: { children: React.ReactNode }) => {
    return (
      <>
        <SidebarProvider>
          <div className="fixed flex w-full z-50">
            <AppSidebar />
            <Header />
          </div>
          <main className="page-container">
            {/* <SidebarTrigger /> */}
            {children}
            <div className="main-container">
              <Outlet />
            </div>
          </main>
        </SidebarProvider>
        {/* <div className="p-2 flex gap-2">
                <Link to="/" className="[&.active]:font-bold text-foreground text-decoration-none">
                    Home
                </Link>{" "}
            </div> */}
        <hr />
        <TanStackRouterDevtools />
      </>
    );
  },
});
