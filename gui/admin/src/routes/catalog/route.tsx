import { createFileRoute, Outlet } from "@tanstack/react-router";
import { getMainCatalogsOptions, getCatalogsOptions } from "shared/src/data/catalog-queries.ts";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  return (
    <>
      <Outlet />
    </>
  );
};

export const Route = createFileRoute("/catalog")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: async ({ context: { queryClient, api_gateway } }) => {
    if (!api_gateway) return;
     await queryClient.ensureQueryData(getMainCatalogsOptions(api_gateway));
     return queryClient.ensureQueryData(getCatalogsOptions(api_gateway));
  },
});
