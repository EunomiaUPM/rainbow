import {Archive, ArrowLeftRight, Feather, Handshake, Users,} from "lucide-react";
import React, {useContext} from "react";
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
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext";


export function AppSidebarProvider() {
    const routerState = useRouterState();
    const {catalog_type} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!

    // Menu items.
    const items = [
        {
            title: "Catalogs",
            url: "/catalog",
            icon: Archive,
        },
        {
            title: "Datahub Catalogs",
            url: "/datahub-catalog",
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


    // @ts-ignore
    const itemsFiltered = items.filter(item => {
        if (catalog_type == "datahub") {
            if (item.title == "Catalogs") return false
        }
        if (catalog_type == "rainbow") {
            if (item.title == "Datahub Catalogs") return false
        }
        return true
    })

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
                            {itemsFiltered.map((item) => (
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
