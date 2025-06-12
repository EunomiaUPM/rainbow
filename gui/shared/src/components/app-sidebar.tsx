import { Calendar, Home, Inbox, Search, Settings } from "lucide-react"
import React from "react" ;
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarTrigger
} from "./ui/sidebar"
import logoImg from "../img/eunomia_logo_lg_light.svg"

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
    icon: Inbox,
  },
  {
    title: "Catalogs",
    url: "/catalog",
    icon: Calendar,
  },
  {
    title: "Transferences",
    url: "/transfer-process",
    icon: Search,
  },
  {
    title: "Contract Negotiation",
    url: "/contract-negotiation",
    icon: Settings,
  },
    {
    title: "Participants",
    url: "/participants",
    icon: Settings,
  }
]

export function AppSidebar() {
  return (
    <Sidebar className="bg-base-sidebar">
      <SidebarContent>
        <SidebarGroup>
          <img src={logoImg} className="h-11 mt-2 mb-4 mr-auto ml-1 flex justify-start object-contain"></img>
          {/* <SidebarGroupLabel>Application</SidebarGroupLabel> */}
          {/* <SidebarTrigger/> */}
      
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <a href={item.url}>
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
  )
}