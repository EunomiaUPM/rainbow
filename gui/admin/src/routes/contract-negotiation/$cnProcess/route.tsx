import { createFileRoute, Outlet } from "@tanstack/react-router";
import { Badge } from "shared/src/components/ui/badge.tsx";
import {
  getContractNegotiationProcessesByCNIDOptions,
  getContractNegotiationMessagesByCNIDOptions,
} from "shared/src/data/contract-queries.ts";
import { formatUrn } from "shared/src/lib/utils.ts";
import { PageHeader } from "shared/src/components/layout/PageHeader";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const { cnProcess } = Route.useParams();
  return (
    <div>
      <PageHeader
        title="Contract negotiation process"
        badge={
          <Badge variant="info" size="lg">
            {formatUrn(cnProcess)}
          </Badge>
        }
      />
      <Outlet />
    </div>
  );
};

/**
 * Route for specific contract negotiation process layout.
 */
export const Route = createFileRoute("/contract-negotiation/$cnProcess")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: async ({ context: { queryClient, api_gateway }, params: { cnProcess } }) => {
    if (!api_gateway) return;
    await queryClient.ensureQueryData(
      getContractNegotiationProcessesByCNIDOptions(api_gateway, cnProcess),
    );
    return queryClient.ensureQueryData(
      getContractNegotiationMessagesByCNIDOptions(api_gateway, cnProcess),
    );
  },
});
