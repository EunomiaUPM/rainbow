import {createFileRoute, Link, Outlet} from "@tanstack/react-router";
import {ArrowLeft} from "lucide-react";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";

const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    const {cnProcess} = Route.useParams();
    return (
        <div>
            <header className="mb-2">
               <Heading level="h3">
                   
                    Contract negotiation process pid: {cnProcess}
               </Heading>
            </header>
            <Outlet/>
        </div>
    );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess")({
    component: RouteComponent,
    notFoundComponent: NotFound,
});
