import {createFileRoute, Link, Outlet} from '@tanstack/react-router'
import {ArrowLeft} from "lucide-react";



const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    return (
        <div className="mb-2">
            <header className="mb-2">
             
            </header>
            <Outlet/>
        </div>
    );
};

export const Route = createFileRoute('/participants/$participantId')({
    component: RouteComponent,
    notFoundComponent: NotFound,
})
