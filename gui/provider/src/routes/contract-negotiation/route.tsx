import { createFileRoute, Link, Outlet } from "@tanstack/react-router";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  return (
    <div className="container mx-auto my-5">
      <header className="mb-2">
        <h2>
          <Link to="/contract-negotiation">Contract negotiation</Link>
        </h2>
      </header>
      <Outlet />
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});
