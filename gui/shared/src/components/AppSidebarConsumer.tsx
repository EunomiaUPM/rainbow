import {Archive, ArrowLeftRight, Feather, Handshake, Home, Users,} from "lucide-react";
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
import logoImg from "./../img/eunomia_logo_lg_light.svg";

// Menu items.
const items = [
    // {
    //     title: "Home",
    //     url: "/",
    //     icon: Home,
    // },
    {
        title: "Provider Catalogs",
        url: "/provider-catalog",
        icon: Archive,
    },

    {
        title: "Contract Negotiation",
        url: "/contract-negotiation",
        icon: Feather,
    },
    {
        title: "Agreements",
        url: "/agreements",
        icon: Handshake,
    },
    {
        title: "Transferences",
        url: "/transfer-process",
        icon: ArrowLeftRight,
    },

    {
        title: "Participants",
        url: "/participants",
        icon: Users,
    },
];

export function AppSidebarConsumer() {
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
