import {createFileRoute, Link, Outlet} from '@tanstack/react-router'
import {ArrowLeft} from "lucide-react";

const NotFound = () => {
    return <div>not found</div>;
};

export const Route = createFileRoute('/catalog/$catalogId')({
    component: RouteComponent,
    notFoundComponent: NotFound
})

function RouteComponent() {
    const {catalogId} = Route.useParams();
    return <div>
        <header className="mb-2">
            <h2 className="flex gap-2 items-center">
                <ArrowLeft className="w-4"/>
                <Link
                    to="/catalog/$catalogId"
                    params={{
                        catalogId: catalogId
                    }}
                >Catalog {catalogId}</Link>
            </h2>
        </header>
        <Outlet/>
    </div>
}
