import { createFileRoute, Outlet } from "@tanstack/react-router";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  return (
    <div>
      <header>hola</header>
      <Outlet />
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});
