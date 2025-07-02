import {createFileRoute, Outlet} from "@tanstack/react-router";


const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
 
    return (
             <>
     
      <Outlet />
    </>
    );
};

export const Route = createFileRoute("/catalog")({
    component: RouteComponent,
    notFoundComponent: NotFound,
});
