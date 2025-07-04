import {createFileRoute, Link, Outlet} from '@tanstack/react-router'
import {ArrowLeft} from "lucide-react";

const NotFound = () => {
    return <div>not found</div>;
};

export const Route = createFileRoute('/datahub-catalog/$catalogId')({
    component: RouteComponent,
    notFoundComponent: NotFound
})

function RouteComponent() {
    const {catalogId} = Route.useParams();
    return <div>
       
        <Outlet/>
    </div>
}
