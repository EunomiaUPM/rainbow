import { createFileRoute, Link, Outlet } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const { transferProcessId } = Route.useParams();
  return (
    <div className=" mb-2">
      <header className="mb-6">
        <Heading
          level="h3"
          className="mb-0.5 font-display flex  gap-3 items-center"
        >
        
          Transfer Process 
              <Badge variant="info" size="lg">
            {" "}
            {transferProcessId.slice(9,29) + "[...]"}{" "}
          </Badge>
        </Heading>
        {/* no borrar */}
        {/* <div className="flex gap-2">
          <p className="text-base mt-[3px]">ID: </p>{" "}
          <Badge variant="info" size="lg">
            {" "}
            {transferProcessId}{" "}
          </Badge>
        </div> */}
      </header>
      <Outlet />
    </div>
  );
};

export const Route = createFileRoute("/transfer-process/$transferProcessId")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});
