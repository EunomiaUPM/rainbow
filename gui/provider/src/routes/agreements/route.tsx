import {createFileRoute, Outlet, useRouterState} from '@tanstack/react-router'
import Heading from "shared/src/components/ui/heading.tsx";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    const routerState = useRouterState();
    console.log("Pathname actual:", routerState.location.pathname);
    return (
        <>
            {/* Evitar que se pinte el titulo de "transferences
    " por duplicado en la pagina de transferencia single */}
            {routerState.location.pathname !== '/agreements' ? null :
                <>
                    <div className="mb-6">
                        <Heading level="h3" className="flex gap-2 items-center">
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