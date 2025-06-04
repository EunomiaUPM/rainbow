import {QueryClient} from "@tanstack/react-query";
import {createRootRouteWithContext, Link, Outlet,} from "@tanstack/react-router";
import {TanStackRouterDevtools} from "@tanstack/router-devtools";

export const Route = createRootRouteWithContext<{
    queryClient: QueryClient;
}>()({
    component: () => {
        return (<>
            <div className="p-2 flex gap-2">
                <Link to="/" className="[&.active]:font-bold text-foreground text-decoration-none">
                    Home
                </Link>{" "}
            </div>
            <hr/>
            <Outlet/>
            <TanStackRouterDevtools/>
        </>)

    }
});
