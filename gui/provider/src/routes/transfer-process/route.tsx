import {createFileRoute, Link, Outlet} from '@tanstack/react-router'
import {ArrowLeft} from "lucide-react";
import  Heading  from "../../../../shared/src/components/ui/heading.tsx";


const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    return (
        <>
            <div className="mb-6">
                <Heading level="h4" className="flex gap-2 items-center">
                    {/* <ArrowLeft className="w-4"/> */}
                    <Link to="/transfer-process">Transfer Processes</Link>
                </Heading>
            </div>
            <Outlet/>
        </>
    );
};

export const Route = createFileRoute('/transfer-process')({
    component: RouteComponent,
    notFoundComponent: NotFound,
})
