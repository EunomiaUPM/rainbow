import {createFileRoute, Link, Outlet} from "@tanstack/react-router";
import {ArrowLeft} from "lucide-react";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    return (
        <div className="container mx-auto my-5">
            <header className="mb-2">
                <h2 className="flex gap-2 items-center">
                    <ArrowLeft className="w-4"/>
                    <Link to="/contract-negotiation">Contract negotiation</Link>
                </h2>
            </header>
            <Outlet/>
        </div>
    );
};

export const Route = createFileRoute("/contract-negotiation")({
    component: RouteComponent,
    notFoundComponent: NotFound,
});
