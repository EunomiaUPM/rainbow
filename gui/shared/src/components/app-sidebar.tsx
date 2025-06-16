import {
  Calendar,
  Home,
  Settings,
  Users,
  Handshake,
  ArrowLeftRight,
  Feather,
  Archive,
} from "lucide-react";
import React from "react";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarTrigger,
} from "./ui/sidebar";
import logoImg from "../img/eunomia_logo_lg_light.svg";
import { useRouterState } from "@tanstack/react-router";

// Menu items.
const items = [
  {
    title: "Home",
    url: "#",
    icon: Home,
  },
  {
    title: "Agreements",
    url: "/agreements",
    icon: Handshake,
  },
  {
    title: "Catalogs",
    url: "/catalog",
    icon: Archive,
  },
  {
    title: "Transferences",
    url: "/transfer-process",
    icon: ArrowLeftRight,
  },
  {
    title: "Contract Negotiation",
    url: "/contract-negotiation",
    icon: Feather,
  },
  {
    title: "Participants",
    url: "/participants",
    icon: Users,
  }
];

export function AppSidebar() {
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
                    <a
                      href={item.url}
                      className={
                        routerState.location.pathname === item.url
                          ? "bg-white/10 text-foreground"
                          : ""
                      }
                    >
                      <item.icon />
                      <span>{item.title}</span>
                    </a>
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
