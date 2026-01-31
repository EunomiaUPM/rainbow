import {createFileRoute, Outlet} from "@tanstack/react-router";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import {Badge} from "shared/src/components/ui/badge.tsx";
import { getContractNegotiationProcessesByCNIDOptions, getContractNegotiationMessagesByCNIDOptions } from "shared/src/data/contract-queries.ts";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const {cnProcess} = Route.useParams();
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
      <Outlet/>
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: async ({ context: { queryClient, api_gateway }, params: {cnProcess} }) => {
     if (!api_gateway) return;
     await queryClient.ensureQueryData(getContractNegotiationProcessesByCNIDOptions(api_gateway, cnProcess));
     return queryClient.ensureQueryData(getContractNegotiationMessagesByCNIDOptions(api_gateway, cnProcess));
  },
});
