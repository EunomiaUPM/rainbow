import { createFileRoute, Outlet, useRouterState } from "@tanstack/react-router";
import Heading from "../../../../shared/src/components/ui/heading.tsx";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const routerState = useRouterState();
  return (
    <>
      {/* Evitar que se pinte el titulo de "transferences
    " por duplicado en la pagina de transferencia single */}
      {routerState.location.pathname !== "/contract-negotiation" ? null : (
        <>
          <div className="mb-6">
            <Heading level="h3" className="flex gap-2 items-center">
              {/* <ArrowLeft className="w-4"/> */}
              Contract Negotiations
            </Heading>
          </div>
        </>
      )}
      <Outlet />
    </>
  );
};

export const Route = createFileRoute("/contract-negotiation")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});
