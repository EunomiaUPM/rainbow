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
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <div className="flex justify-between">
          <SidebarGroupLabel>Application</SidebarGroupLabel>
          {/* <SidebarTrigger/> */}
          </div>
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