import {createFileRoute, Outlet, useRouterState} from "@tanstack/react-router";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import { getParticipantsOptions } from "shared/src/data/participant-queries.ts";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const routerState = useRouterState();
  return (
    <>
      {/* Evitar que se pinte el titulo de "transferences
    " por duplicado en la pagina de transferencia single */}
      {routerState.location.pathname !== "/participants" ? null : (
        <>
          <div className="mb-6">
            <Heading level="h3" className="flex gap-2 items-center">
              {/* <ArrowLeft className="w-4"/> */}
              Participants
            </Heading>
          </div>
        </>
      )}
      <Outlet/>
    </>
  );
};

export const Route = createFileRoute("/participants")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: ({ context: { queryClient, api_gateway } }) => {
      if (!api_gateway) return;
      return queryClient.ensureQueryData(getParticipantsOptions(api_gateway));
  },
});
