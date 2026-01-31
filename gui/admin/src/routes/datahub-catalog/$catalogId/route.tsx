import { createFileRoute, useRouterState, Outlet } from "@tanstack/react-router";
import { Badge } from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";
import { formatUrn } from "shared/src/lib/utils.ts";

const NotFound = () => {
  return <div>not found</div>;
};

export const Route = createFileRoute("/datahub-catalog/$catalogId")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});

function RouteComponent() {
  const { catalogId } = Route.useParams();
  const routerState = useRouterState();
  // formatear id del catalogo para que sea igual que el pathname
  const catalogIdURL = "/datahub-catalog/" + catalogId.replace(/:/g, "%3A");

  return (
    <div>
      <header className="mb-2">
        {routerState.location.pathname !== `${catalogIdURL}` ? null : (
          <Heading level="h3" className="flex gap-2 items-center">
            Catalog
            <Badge variant="info" size="lg">
              {formatUrn(catalogId)}
            </Badge>
          </Heading>
        )}
      </header>
      <Outlet />
    </div>
  );
}
