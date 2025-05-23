import {createFileRoute, Link, Outlet} from "@tanstack/react-router";
import {ArrowLeft} from "lucide-react";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    const {cnProcess} = Route.useParams();
    return (
        <div>
            <header className="mb-2">
                <h2 className="flex gap-2 items-center">
                    <ArrowLeft className="w-4"/>
                    <Link
                        to="/contract-negotiation/$cnProcess"
                        params={{cnProcess: cnProcess}}
                    >Contract negotiation process pid: {cnProcess}</Link>
                </h2>
            </header>
            <Outlet/>
        </div>
    );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess")({
    component: RouteComponent,
    notFoundComponent: NotFound,
});
