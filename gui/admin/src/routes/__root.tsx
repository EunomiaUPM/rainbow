import { QueryClient } from "@tanstack/react-query";
import { createRootRouteWithContext, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { SidebarInset, SidebarProvider, SidebarTrigger } from "../../../shared/src/components/ui/sidebar";
import React from "react";
import { AppSidebarProvider } from "shared/src/components/AppSidebarProvider.tsx";
import { Header } from "shared/src/components/header.tsx";

import { GeneralErrorComponent } from "../components/GeneralErrorComponent";

import { GlobalLoadingIndicator } from "../components/GlobalLoadingIndicator";

/**
 * Root route component providing application shell, sidebar, and global providers.
 */
export const Route = createRootRouteWithContext<{
    queryClient: QueryClient;
    api_gateway: string;
}>()({
    component: ({ children }: { children: React.ReactNode }) => {
        const widthPage = window.innerWidth;
        return (
            <>
                <SidebarProvider>
                    <AppSidebarProvider />
                    <SidebarInset>
                        <Header />
                        <div className="flex flex-1 flex-col gap-4 p-8 overflow-hidden items-start justify-start w-full h-full">
                            <Outlet />
                        </div>
                    </SidebarInset>
                </SidebarProvider>
                <GlobalLoadingIndicator />
                <TanStackRouterDevtools />
            </>
        );
    },
    errorComponent: GeneralErrorComponent,
});
