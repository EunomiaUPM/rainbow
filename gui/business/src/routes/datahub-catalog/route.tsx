import {createFileRoute, Outlet, redirect} from "@tanstack/react-router";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    
    return (
        <>
            <Outlet/>
        </>
    );
};

export const Route = createFileRoute("/datahub-catalog")({
    component: RouteComponent,
    notFoundComponent: NotFound,
    beforeLoad: ({context}) => {
        // @ts-ignore
        if (!context.auth.isAuthenticated) {
            throw redirect({
                to: '/login', // Redirige a la página de login si no está autenticado
                search: {
                    redirect: location.pathname, // Opcional: para redirigir de vuelta después del login
                },
            });
        }
    },
});
