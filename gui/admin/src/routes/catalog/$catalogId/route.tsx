import { createFileRoute, Outlet, useRouterState } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import { Badge } from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";
import { getCatalogsByIdOptions, getDatasetsByCatalogIdOptions, getDataServicesByCatalogIdOptions } from "shared/src/data/catalog-queries.ts";

const NotFound = () => {
  return <div>not found</div>;
};

/**
 * Route for specific catalog layout.
 */
export const Route = createFileRoute("/catalog/$catalogId")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: async ({ context: { queryClient, api_gateway }, params: { catalogId } }) => {
    if (!api_gateway) return;
    await queryClient.ensureQueryData(getCatalogsByIdOptions(api_gateway, catalogId));
    await queryClient.ensureQueryData(getDatasetsByCatalogIdOptions(api_gateway, catalogId));
    return queryClient.ensureQueryData(getDataServicesByCatalogIdOptions(api_gateway, catalogId));
  },
});

function RouteComponent() {
  const { catalogId } = Route.useParams();
  const routerState = useRouterState();


  const catalogIdURL = "/catalog/" + catalogId.replace(/:/g, "%3A");

  return (
    <div className="h-full w-full flex flex-col">
      {routerState.location.pathname !== `${catalogIdURL}` ? null : (
        <header className="mb-2 shrink-0">
          <Heading level="h3" className="flex gap-2 items-center">
            Catalog
            <Badge variant="info" size="lg">
              {formatUrn(catalogId)}
            </Badge>
          </Heading>
        </header>
      )}
      <Outlet />
    </div>
  );
}
