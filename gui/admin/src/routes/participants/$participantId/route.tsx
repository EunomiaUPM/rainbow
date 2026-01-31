import { createFileRoute, Outlet } from "@tanstack/react-router";
import { getParticipantByIdOptions } from "shared/src/data/participant-queries.ts";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  return (
    <div className="mb-2">
      <header className="mb-2"></header>
      <Outlet />
    </div>
  );
};

export const Route = createFileRoute("/participants/$participantId")({
  component: RouteComponent,
  notFoundComponent: NotFound,
  loader: ({ context: { queryClient, api_gateway }, params: {participantId} }) => {
      if (!api_gateway) return;
      return queryClient.ensureQueryData(getParticipantByIdOptions(api_gateway, participantId));
  },
});
