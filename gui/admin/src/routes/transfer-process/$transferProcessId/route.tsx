import {createFileRoute, Outlet} from "@tanstack/react-router";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import {Badge} from "shared/src/components/ui/badge.tsx";
import { getTransferProcessByIdOptions } from "shared/src/data/transfer-queries.ts";
import { formatUrn } from "shared/src/lib/utils.ts";
import { PageHeader } from "shared/src/components/layout/PageHeader";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const {transferProcessId} = Route.useParams();
  return (
    <div className=" mb-2">
      <PageHeader
        className="mb-6"
        title="Transfer Process"
        badge={<Badge variant="info" size="lg">{formatUrn(transferProcessId)}</Badge>}
      />
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
