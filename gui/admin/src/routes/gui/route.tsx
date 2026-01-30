import { createFileRoute, Outlet } from "@tanstack/react-router";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  return (
    <div className="h-screen bg-pink-200">
      <Outlet />
    </div>
  );
};

export const Route = createFileRoute("/gui")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});
