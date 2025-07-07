import { createFileRoute, Link, Outlet } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const { cnProcess } = Route.useParams();
  return (
    <div>
      <header className="mb-2">
        <Heading level="h3" className="mb-0.5 font-display flex  gap-3 items-center">
          Contract negotiation process
          <Badge variant="info" size="lg">
            {" "}
            {cnProcess.slice(9, 29) + "[...]"}{" "}
          </Badge>
        </Heading>
      </header>
      <Outlet />
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});
