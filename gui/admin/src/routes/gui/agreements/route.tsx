import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/gui/agreements")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div>
      <h1>Agreements</h1>
      <Outlet />
    </div>
  );
}
