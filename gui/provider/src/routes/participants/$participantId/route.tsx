import { createFileRoute, Outlet } from "@tanstack/react-router";

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
});
