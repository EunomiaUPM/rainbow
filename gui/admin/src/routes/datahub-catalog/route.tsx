import { createFileRoute, Outlet } from "@tanstack/react-router";
import { getGetDatahubDomainsQueryOptions } from "shared/src/data/orval/datahub/datahub";

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

/**
 * Datahub Catalog route layout and data loading.
 */
export const Route = createFileRoute("/datahub-catalog")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: ({ context: { queryClient } }) => {
    return queryClient.ensureQueryData(getGetDatahubDomainsQueryOptions());
  },
});
