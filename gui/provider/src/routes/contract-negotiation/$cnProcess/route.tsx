import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/contract-negotiation/$cnProcess")({
  component: RouteComponent,
});

function RouteComponent() {
  return <div>
    <header>hola</header>
    <Outlet />
  </div>;
}
