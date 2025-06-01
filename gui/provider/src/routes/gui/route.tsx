import {createFileRoute, Outlet} from '@tanstack/react-router'


const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    return (
        <div className="container mx-auto my-5">
            <Outlet/>
        </div>
    );
};

export const Route = createFileRoute('/gui')({
    component: RouteComponent,
    notFoundComponent: NotFound,
})
