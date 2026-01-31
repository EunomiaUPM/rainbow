import { createFileRoute, Link, Outlet } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";

const NotFound = () => {
  return <div>not found</div>;
};

const RouteComponent = () => {
  return (
    <div className="container mx-auto my-5">
      <header className="mb-2">
        <h2 className="flex gap-2 items-center">
          <ArrowLeft className="w-4" />
          <Link to="/subscriptions">Subscriptions</Link>
        </h2>
      </header>
      <Outlet />
    </div>
  );
};

/**
 * Subscriptions route layout.
 */
export const Route = createFileRoute("/subscriptions")({
  component: RouteComponent,
  notFoundComponent: NotFound,
});
