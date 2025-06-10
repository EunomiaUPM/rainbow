import {createFileRoute, Link, Outlet} from '@tanstack/react-router'
import {ArrowLeft} from "lucide-react";
import  Heading  from "../../../../../shared/src/components/ui/heading.tsx";

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "../../../../../shared/src/components/ui/breadcrumb.tsx";


const NotFound = () => {
    return <div>not found</div>;
};

const RouteComponent = () => {
    const {transferProcessId} = Route.useParams()
    return (
        <div className="mb-2">
            <Breadcrumb>
              <BreadcrumbList>
             
                <BreadcrumbItem>
                  <BreadcrumbLink href="/transfer-process">Transferences</BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                  <BreadcrumbPage>Transference {transferProcessId}</BreadcrumbPage>
                </BreadcrumbItem>
              </BreadcrumbList>
            </Breadcrumb>
            <header className="mb-2">
                <Heading level="h1" className="font-display flex gap-2 items-center">
                    {/* <ArrowLeft className="w-4"/> */}
                    <Link
                        to="/transfer-process/$transferProcessId"
                        params={{
                            transferProcessId: transferProcessId
                        }}
                    >Transfer Process Id {transferProcessId}</Link>
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
