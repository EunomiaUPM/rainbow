import {createFileRoute, Link, Outlet} from '@tanstack/react-router'
import {ArrowLeft} from "lucide-react";


const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    return (
        <div className="mb-2">
            <header className="mb-2">
                <h2 className="flex gap-2 items-center">
                    <ArrowLeft className="w-4"/>
                    <Link
                        to="/participants"

                    >Participants</Link>
                </h2>
            </header>
            <Outlet/>
        </div>
    );
};

export const Route = createFileRoute('/participants/$participantId')({
    component: RouteComponent,
    notFoundComponent: NotFound,
})
