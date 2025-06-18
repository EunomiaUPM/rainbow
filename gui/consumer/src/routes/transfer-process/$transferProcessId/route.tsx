import {createFileRoute, Outlet} from '@tanstack/react-router'
import Heading from "../../../../../shared/src/components/ui/heading.tsx";


const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    const {transferProcessId} = Route.useParams()
    return (
        <div className=" mb-2">

            <header className="mb-2">
                <Heading level="h3" className="font-display flex gap-2 items-center">
                    {/* <ArrowLeft className="w-4"/> */}
                    Transfer Process Id {transferProcessId}
                </Heading>
            </header>
            <Outlet/>
        </div>
    );
};

export const Route = createFileRoute('/transfer-process/$transferProcessId')({
    component: RouteComponent,
    notFoundComponent: NotFound,
})
