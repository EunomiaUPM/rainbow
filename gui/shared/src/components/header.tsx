import * as React from "react";
import {useContext} from "react";
import {Link, useRouterState} from "@tanstack/react-router";

import {Breadcrumb, BreadcrumbItem, BreadcrumbList, BreadcrumbSeparator,} from "shared/src/components/ui/breadcrumb";
import NotificationsIcon from "@mui/icons-material/Notifications";
import PersonIcon from "@mui/icons-material/Person";
import {AuthContext, AuthContextType} from "shared/src/context/AuthContext";
import {Button} from "shared/src/components/ui/button";
import {LogOut} from "lucide-react";

const Header = () => {
    const routerState = useRouterState();
    const {isAuthenticated, unsetAuthentication} =
        useContext<AuthContextType | null>(AuthContext)!;

    let paths = routerState.location.pathname.split("/");
    let formatPath = (path) => {
        // si el path es un single, y va por id, quitarle las primeras litras
        if (path.includes("urn")) {
            let formattedPath = path.slice(13, 24) + "[...]";
            return formattedPath;
        } else {
            let formattedPath = path.split("-").join(" ");
            return formattedPath;
        }
    };
    paths.forEach((path, index) => {
        // console.log(`Path ${index}:`, path);
    });
    paths.splice(0, 1); // Eliminar el primer elemento vacío
    // console.log(formatPath(paths[0]), " formatted path");
    return (
        <div
            className=" bg-background w-full border-b py-1.5 z-50 border-stroke px-4 flex justify-between items-center">
            <Breadcrumb>
                <BreadcrumbList>
                    {paths.map((path, index) => (
                        <span key={path} className="contents">
                            {/* Este condicional es importante porque sino sale un breadcrumb aislado
            que no lleva a ninguna parte de dataset o dataservice. No pinta este breadcrumb */}
                            {/* Sólo en consumer aplica el mismo principio para catalog (Provider.-catalog solo existe
            en consumer) Si paths contiene provider-catalog, entonces "catalog" no pinta un breadcrumb */}
                            {path === "dataset" ||
                            path === "data-service" ||
                            (paths.includes("provider-catalog") && path == "catalog") ? (
                                ""
                            ) : (
                                <>
                                    <BreadcrumbItem className="max-w-40 truncate">
                                        {/*<BreadcrumbLink*/}
                                        {/*  // coger el link del path, sumando los paths*/}
                                        {/*  href={`/${paths.slice(0, index + 1).join("/")}`}*/}
                                        {/*>*/}
                                        {/*  {path.includes("urn") &&*/}
                                        {/*    formatPath(paths.slice(index - 1, index) + " ")}*/}
                                        {/*  {formatPath(path)}*/}
                                        {/*  /!* {console.log(path, "incluye" ,path.includes("urn"), "o no?")} *!/*/}
                                        {/*</BreadcrumbLink>*/}
                                        <Link href={`/${paths.slice(0, index + 1).join("/")}`}>
                                            {path.includes("urn") &&
                                                formatPath(paths.slice(index - 1, index) + " ")}
                                            {formatPath(path)}
                                        </Link>
                                    </BreadcrumbItem>
                                    {index < paths.length - 1 ? <BreadcrumbSeparator/> : ""}
                                </>
                            )}
                        </span>
                    ))}
                </BreadcrumbList>
            </Breadcrumb>
            <div className="flex flex-row gap-4">
                <Link to="/subscriptions">
                    <NotificationsIcon className="cursor-pointer"/>
                </Link>
                <Link to="">
                    <PersonIcon className="cursor-pointer"/>
                </Link>
                {isAuthenticated && (
                    <Button variant="ghost" size="xs" onClick={() => unsetAuthentication()}>
                        Logout
                        <LogOut/>
                    </Button>
                )}
                {/* <Link to="/user">User</Link> */}
            </div>
        </div>
    );
};

export {Header};
