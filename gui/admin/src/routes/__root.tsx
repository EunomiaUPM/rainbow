import {QueryClient} from "@tanstack/react-query";
import {createRootRouteWithContext, Outlet} from "@tanstack/react-router";
import {TanStackRouterDevtools} from "@tanstack/router-devtools";
import {SidebarProvider, SidebarTrigger} from "../../../shared/src/components/ui/sidebar";
import React from "react";
import {AppSidebarProvider} from "shared/src/components/AppSidebarProvider.tsx";
import {Header} from "shared/src/components/header.tsx";

import { GeneralErrorComponent } from "../components/GeneralErrorComponent";

import {GlobalLoadingIndicator} from "../components/GlobalLoadingIndicator";

export const Route = createRootRouteWithContext<{
    queryClient: QueryClient;
    api_gateway: string;
}>()({
    component: ({children}: { children: React.ReactNode }) => {
        const widthPage = window.innerWidth;
        return (
            <>
                <SidebarProvider>
                    <div className="fixed flex w-full z-50">
                        <AppSidebarProvider/>
                        <Header/>
                    </div>
                    <main className="page-container">
                        {widthPage < 768 ? <SidebarTrigger/> : ""}
                        {children}
                        <div className="main-container">
                            <Outlet/>
                        </div>
                    </main>
                </SidebarProvider>
                <GlobalLoadingIndicator />
                <TanStackRouterDevtools/>
            </>
        );
    },
    errorComponent: GeneralErrorComponent,
});
