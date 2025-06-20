import {createFileRoute, Outlet, useRouterState} from "@tanstack/react-router";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    const routerState = useRouterState();
    return (
        <>
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

export const Route = createFileRoute("/datahub-catalog")({
    component: RouteComponent,
    notFoundComponent: NotFound,
});
