import {createFileRoute, Outlet, useRouterState} from "@tanstack/react-router";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import { getTransferProcessesOptions } from "shared/src/data/transfer-queries.ts";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const routerState = useRouterState();
  return (
    <>
      {routerState.location.pathname !== "/transfer-process" ? null : (
        <>
          <div className="mb-6">
            <Heading level="h3" className="flex gap-2 items-center">
              Transfer Processes
            </Heading>
          </div>
        </>
      )}
      <Outlet/>
    </>
  );
};

export const Route = createFileRoute("/transfer-process")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: ({ context: { queryClient, api_gateway } }) => {
    if (!api_gateway) return;
    return queryClient.ensureQueryData(getTransferProcessesOptions(api_gateway));
  },
});
