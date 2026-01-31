import React, { useContext, useMemo } from "react";
import { Link, useRouterState } from "@tanstack/react-router";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "shared/src/components/ui/breadcrumb";
import NotificationsIcon from "@mui/icons-material/Notifications";
import PersonIcon from "@mui/icons-material/Person";
import { AuthContext, AuthContextType } from "shared/src/context/AuthContext";
import { Button } from "shared/src/components/ui/button";
import { LogOut } from "lucide-react";
import { formatUrn } from "shared/src/lib/utils";

/**
 * Header component containing breadcrumbs and user actions.
 */
export const Header = () => {
  const routerState = useRouterState();
  const { isAuthenticated, unsetAuthentication } = useContext<AuthContextType | null>(AuthContext)!;

  const breadcrumbs = useMemo(() => {
    const rawPaths = routerState.location.pathname.split("/").filter((p) => p !== "");

    const isVisible = (segment: string, allSegments: string[]) => {
      if (segment === "dataset") return false;
      if (segment === "data-service") return false;
      if (segment === "catalog" && allSegments.includes("provider-catalog")) return false;
      return true;
    };

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

  return (
    <div className="bg-background w-full border-b py-1.5 z-50 border-stroke px-4 flex justify-between items-center">
      <Breadcrumb>
        <BreadcrumbList>
          {breadcrumbs.map((item, index) => {
            const isLast = index === breadcrumbs.length - 1;

            return (
              <React.Fragment key={item.key}>
                <BreadcrumbItem className="max-w-40 truncate">
                  {isLast ? (
                    <BreadcrumbPage>{item.label}</BreadcrumbPage>
                  ) : (
                    <BreadcrumbLink asChild>
                      <Link to={item.href}>
                        {item.label}
                      </Link>
                    </BreadcrumbLink>
                  )}
                </BreadcrumbItem>
                {!isLast && <BreadcrumbSeparator />}
              </React.Fragment>
            );
          })}
        </BreadcrumbList>
      </Breadcrumb>
      <div className="flex flex-row gap-4">
        <Link to="/subscriptions">
          <NotificationsIcon className="cursor-pointer text-muted-foreground hover:text-foreground transition-colors" />
        </Link>

        <Link to="">
          <PersonIcon className="cursor-pointer text-muted-foreground hover:text-foreground transition-colors" />
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
