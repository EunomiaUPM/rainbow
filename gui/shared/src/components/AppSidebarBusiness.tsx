import {Apple, Archive, ChartBarIncreasing} from "lucide-react";
import React from "react";
import {Link, useRouterState} from "@tanstack/react-router"
import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "./ui/sidebar";
// @ts-ignore
import logoImg from "./../img/eunomia_logo_lg_light.svg";

// Menu items.
const items = [
    {
        title: "Datahub Catalogs",
        url: "/datahub-catalog",
        icon: Archive,
    },
    {
        title: "Data Access Requests",
        url: "/requests",
        icon: Apple,
    },
    {
        title: "Dashboard",
        url: "/dashboard",
        icon: ChartBarIncreasing,
    },
];

export function AppSidebarBusiness() {
    const routerState = useRouterState();
    // console.log("Router state in SidebarMenuItem:", routerState.location.pathname);

    return (
        <Sidebar className="bg-base-sidebar">
            <SidebarContent>
                <SidebarGroup>
                    <img
                        src={logoImg}
                        className="h-11 mt-2 mb-4 mr-auto ml-1 flex justify-start object-contain"
                    ></img>
                    {/* <SidebarGroupLabel>Application</SidebarGroupLabel> */}
                    {/* <SidebarTrigger/> */}

                    <SidebarGroupContent>
                        <SidebarMenu>
                            {items.map((item) => (
                                <SidebarMenuItem key={item.title}>
                                    <SidebarMenuButton asChild>
                                        <Link to={item.url} className={
                                            routerState.location.pathname === item.url
                                                ? "bg-white/10 text-white"
                                                : ""
                                        }>
                                            <item.icon/>
                                            <span>{item.title}</span>
                                        </Link>

                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            ))}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    );
}
