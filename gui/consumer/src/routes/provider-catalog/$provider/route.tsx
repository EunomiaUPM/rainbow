import {createFileRoute, Outlet} from "@tanstack/react-router";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {

    return (
        <>
            {/* Evitar que se pinte el titulo de "transferences
    " por duplicado en la pagina de transferencia single */}
           
            <Outlet/>
        </>
    );
};

export const Route = createFileRoute("/provider-catalog/$provider")({
    component: RouteComponent,
    notFoundComponent: NotFound,
});
