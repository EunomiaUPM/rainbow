import { createFileRoute, Link, Outlet } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import { useRouterState } from "@tanstack/react-router";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  const routerState = useRouterState();
  return (
    <>
      {/* Evitar que se pinte el titulo de "transferences
    " por duplicado en la pagina de transferencia single */}
      {routerState.location.pathname !== "/agreements" ? null : (
        <>
          <div className="mb-6">
            <Heading level="h3" className="flex gap-2 items-center">
              {/* <ArrowLeft className="w-4"/> */}
              Agreements
            </Heading>
            {/* <p className="w-[75ch] text-sm"> 
                            Here you can find the established agreements with the provider that allow you to request transferences for the requested datasets and/or services, in compliance with the defined policies.
                        </p> */}
            <p className="w-[75ch] text-sm">
              A succesful contract negotiation with a provider will result in an agreement that
              allows you to request transferences for the requested datasets and/or services, in
              compliance with the defined policies.
            </p>
          </div>
        </>
      )}
      <Outlet />
    </>
  );
};

export const Route = createFileRoute("/agreements")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});
