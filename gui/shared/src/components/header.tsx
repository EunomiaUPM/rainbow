/**
 * header.tsx
 *
 * Main application header component that provides navigation breadcrumbs
 * and user action controls. Positioned at the top of the content area.
 *
 * Features:
 * - Dynamic breadcrumb generation from current route
 * - Responsive design (collapses intermediate breadcrumbs on mobile)
 * - User authentication status display
 * - Quick access to notifications and user profile
 *
 * @example
 * // Used in the root layout
 * <Header />
 */

import React, { useContext, useMemo } from "react";
import { Link, useRouterState } from "@tanstack/react-router";
import { cn } from "shared/src/lib/utils";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "shared/src/components/ui/breadcrumb";
import { AuthContext, AuthContextType } from "shared/src/context/AuthContext";
import { Button } from "shared/src/components/ui/button";
import { LogOut, Bell, User } from "lucide-react";
import { formatUrn } from "shared/src/lib/utils";
import { SidebarTrigger } from "shared/src/components/ui/sidebar";
import { Separator } from "shared/src/components/ui/separator";

// =============================================================================
// HEADER COMPONENT
// =============================================================================

/**
 * Application header with breadcrumb navigation and user controls.
 *
 * The breadcrumb trail is automatically generated from the current URL path,
 * with smart formatting for URN segments and responsive visibility rules
 * that hide intermediate items on mobile devices.
 *
 * @returns The header component with sidebar trigger, breadcrumbs, and user actions
 */
export const Header = () => {
  const routerState = useRouterState();
  const { isAuthenticated, unsetAuthentication } = useContext<AuthContextType | null>(AuthContext)!;

  // ---------------------------------------------------------------------------
  // Breadcrumb Generation
  // ---------------------------------------------------------------------------

  /**
   * Generates breadcrumb items from the current route path.
   * Handles URN formatting and filters out certain navigation segments
   * that shouldn't appear in the breadcrumb trail.
   */
  const breadcrumbs = useMemo(() => {
    const rawPaths = routerState.location.pathname.split("/").filter((p) => p !== "");

    /**
     * Determines if a path segment should be visible in the breadcrumb.
     * Some segments are implementation details and shouldn't be shown.
     */
    const isVisible = (segment: string, allSegments: string[]) => {
      if (segment === "dataset") return false;
      if (segment === "data-service") return false;
      if (segment === "catalog" && allSegments.includes("provider-catalog")) return false;
      return true;
    };

    /**
     * Formats a path segment for display.
     * URNs are truncated, dashes are replaced with spaces.
     */
    const formatLabel = (segment: string, prevSegment: string | undefined) => {
      if (segment.includes("urn")) {
        return formatUrn(segment);
      }
      return segment.split("-").join(" ");
    };

    const items = [];
    let currentPath = "";

    for (let i = 0; i < rawPaths.length; i++) {
      const segment = rawPaths[i];
      const decodedSegment = decodeURIComponent(segment);
      currentPath += `/${segment}`;

      if (isVisible(decodedSegment, rawPaths)) {
        items.push({
          key: currentPath,
          href: currentPath,
          label: formatLabel(decodedSegment, rawPaths[i - 1]),
          originalSegment: segment,
        });
      }
    }
    return items;
  }, [routerState.location.pathname]);

  // ---------------------------------------------------------------------------
  // Render
  // ---------------------------------------------------------------------------

  return (
    <div className="bg-background w-full border-b border-white/5 py-1 z-50 h-9 px-3 flex justify-between items-center gap-4">
      {/* Left section: Sidebar trigger and breadcrumbs */}
      <div className="flex items-center gap-2 overflow-hidden">
        <SidebarTrigger className="h-6 w-6 shrink-0" />
        <Separator orientation="vertical" className="h-4 shrink-0" />
        <Breadcrumb className="overflow-hidden">
          <BreadcrumbList className="flex-nowrap whitespace-nowrap">
            {breadcrumbs.map((item, index) => {
              const isLast = index === breadcrumbs.length - 1;
              const isFirst = index === 0;
              // On mobile, only show first and last items
              const isHiddenOnMobile = !isFirst && !isLast;

              return (
                <React.Fragment key={item.key}>
                  <BreadcrumbItem className={cn("truncate max-w-[150px] md:max-w-xs",
                    isHiddenOnMobile ? "hidden md:inline-flex" : "inline-flex"
                  )}>
                    {/* Show ellipsis on mobile for collapsed items */}
                    {isHiddenOnMobile && index === 1 && breadcrumbs.length > 2 && (
                      <span className="md:hidden mx-1">...</span>
                    )}
                    {!isHiddenOnMobile && (
                      isLast ? (
                        <BreadcrumbPage className="truncate">{item.label}</BreadcrumbPage>
                      ) : (
                        <BreadcrumbLink asChild className="text-xs text-muted-foreground hover:text-foreground transition-colors truncate">
                          <Link to={item.href}>
                            {item.label}
                          </Link>
                        </BreadcrumbLink>
                      )
                    )}
                  </BreadcrumbItem>
                  {!isLast && (
                    <BreadcrumbSeparator className={cn(isHiddenOnMobile ? "hidden md:flex" : "flex")} />
                  )}
                </React.Fragment>
              );
            })}
          </BreadcrumbList>
        </Breadcrumb>
      </div>

      {/* Right section: User actions */}
      <div className="flex flex-row gap-4 shrink-0">
        <Link to="/subscriptions">
          <Bell className="cursor-pointer text-muted-foreground hover:text-foreground transition-colors h-5 w-5" />
        </Link>

        <Link to="">
          <User className="cursor-pointer text-muted-foreground hover:text-foreground transition-colors h-5 w-5" />
        </Link>
        {isAuthenticated && (
          <Button variant="ghost" size="xs" onClick={() => unsetAuthentication()}>
            Logout
            <LogOut className="ml-2 h-4 w-4" />
          </Button>
        )}
      </div>
    </div>
  );
};
