import { createFileRoute, useRouterState, Outlet } from "@tanstack/react-router";
import { Badge } from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";

const NotFound = () => {
  return <div>not found</div>;
};

export const Route = createFileRoute("/provider-catalog/$provider/catalog/$catalogId")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});

function RouteComponent() {
  const { catalogId } = Route.useParams();
  //     const routerState = useRouterState();
  //     const catalogIdURL = "/provider-catalog/" + catalogId.replace(/:/g, "%3A") ;
  //  console.log(catalogId, "catalogId");
  //  console.log(catalogIdURL, "catalogIdURL");
  //  console.log(routerState.location.pathname, "router catalog 2?");
  //  console.log(catalogIdURL === routerState.location.pathname ? "vamoooos" : "todavia no")
  return (
    <div>
      {/* <header className="mb-2">
        {routerState.location.pathname !== `${catalogIdURL}` ? null : (
          <Heading level="h3" className="flex gap-2 items-center">
            Catalog
            <Badge variant="info" size="lg">
              {catalogId.slice(9, 29) + "[...]"}
            </Badge>
          </Heading>
        )}
      </header> */}
      <Outlet />
    </div>
  );
}
