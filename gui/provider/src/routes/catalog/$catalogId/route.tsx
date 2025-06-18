import {createFileRoute, Link, Outlet} from '@tanstack/react-router'
import {ArrowLeft} from "lucide-react";
import { Badge } from 'shared/src/components/ui/badge';
import Heading from 'shared/src/components/ui/heading';

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
            <Heading level="h3"  className="flex gap-2 items-center">
                Catalog 
                <Badge variant="info" size="lg">
                {catalogId.slice(9, 29) + "[...]"}
                </Badge>
                </Heading>
      
        </header>
        <Outlet/>
    </div>
}
