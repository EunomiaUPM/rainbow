import { createFileRoute, Outlet, useRouterState } from "@tanstack/react-router";
import Heading from "shared/src/components/ui/heading.tsx";
import { getAgreementsOptions } from "shared/src/data/agreement-queries";

const NotFound = () => {
  return <div>not found</div>;
};

/**
 * Agreements route component.
 */
const RouteComponent = () => {
  const routerState = useRouterState();
  return (
    <>
      {routerState.location.pathname !== "/agreements" ? null : (
        <>
          <div className="mb-6">
            <Heading level="h3" className="flex gap-2 items-center">
              Agreements
            </Heading>
          </div>
        </>
      )}
      <Outlet />
    </>
  );
};

/**
 * Agreements route layout.
 */
export const Route = createFileRoute("/agreements")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: ({ context: { queryClient, api_gateway } }) => {
    if (!api_gateway) return;
    return queryClient.ensureQueryData(getAgreementsOptions(api_gateway));
  },
});
