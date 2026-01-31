import { createFileRoute, Outlet } from "@tanstack/react-router";
import { getDatahubCatalogsOptions } from "shared/src/data/datahub-catalog-queries.ts";

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

export const Route = createFileRoute("/datahub-catalog")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: ({ context: { queryClient, api_gateway } }) => {
      if (!api_gateway) return;
      return queryClient.ensureQueryData(getDatahubCatalogsOptions(api_gateway));
  },
});
