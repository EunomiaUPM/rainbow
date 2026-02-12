import { createFileRoute, Outlet, useRouterState } from "@tanstack/react-router";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import { getGetAllParticipantsQueryOptions } from "shared/src/data/orval/participants/participants";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const routerState = useRouterState();
  return (
    <>
      {routerState.location.pathname !== "/participants" ? null : (
        <div className="mb-6">
          <Heading level="h3" className="flex gap-2 items-center">
            Participants
          </Heading>
        </div>
      )}
      <Outlet />
    </>
  );
};

/**
 * Participants route layout and data loading.
 */
export const Route = createFileRoute("/participants")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: ({ context: { queryClient } }) => {
    return queryClient.ensureQueryData(getGetAllParticipantsQueryOptions());
  },
});
