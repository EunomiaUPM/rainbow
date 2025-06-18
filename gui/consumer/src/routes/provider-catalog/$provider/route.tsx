import {createFileRoute, Outlet, useRouterState} from "@tanstack/react-router";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    const routerState = useRouterState();
    return (
        <>
            {/* Evitar que se pinte el titulo de "transferences
    " por duplicado en la pagina de transferencia single */}
            {routerState.location.pathname !== "/catalog" ? null : (
                <>
                    {/* <div className="mb-6">
            <Heading level="h4" className="flex gap-2 items-center">

             Main Catalog
            </Heading>
          </div> */}
                </>
            )}
            <Outlet/>
        </>
    );
};

export const Route = createFileRoute("/provider-catalog/$provider")({
    component: RouteComponent,
    notFoundComponent: NotFound,
});
