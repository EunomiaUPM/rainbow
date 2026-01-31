import {createFileRoute, Outlet} from "@tanstack/react-router";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import {Badge} from "shared/src/components/ui/badge.tsx";
import { getTransferProcessByIdOptions } from "shared/src/data/transfer-queries.ts";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const {transferProcessId} = Route.useParams();
  return (
    <div className=" mb-2">
      <header className="mb-6">
        <Heading level="h3" className="mb-0.5 font-display flex  gap-3 items-center">
          Transfer Process
          <Badge variant="info" size="lg">
            {" "}
            {transferProcessId.slice(9, 29) + "[...]"}{" "}
          </Badge>
        </Heading>
      </header>
      <Outlet/>
    </div>
  );
};

export const Route = createFileRoute("/transfer-process/$transferProcessId")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: ({ context: { queryClient, api_gateway }, params: {transferProcessId} }) => {
      if (!api_gateway) return;
      return queryClient.ensureQueryData(getTransferProcessByIdOptions(api_gateway, transferProcessId));
  },
});
