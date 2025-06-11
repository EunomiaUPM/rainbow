import {createFileRoute, Link, Outlet} from '@tanstack/react-router'
import {ArrowLeft} from "lucide-react";
import  Heading  from "../../../../shared/src/components/ui/heading.tsx";
import {   useRouterState, } from "@tanstack/react-router";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
     const routerState = useRouterState();
     console.log("Pathname actualch:", routerState.location.pathname);
    return (
        <>
      {/* Evitar que se pinte el titulo de "transferences
    " por duplicado en la pagina de transferencia single */}
        {routerState.location.pathname !== '/agreements' ? null : 
            <>
            <div className="mb-6">
                <Heading level="h4" className="flex gap-2 items-center">
                    {/* <ArrowLeft className="w-4"/> */}
                   Agreements
                </Heading>
            </div>
             </>
             }
             <Outlet/>
        </>
    );
};

export const Route = createFileRoute('/agreements')({
    component: RouteComponent,
    notFoundComponent: NotFound
})