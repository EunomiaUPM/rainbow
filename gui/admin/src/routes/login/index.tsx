import { createFileRoute } from "@tanstack/react-router";

/**
 * Route for the login page.
 */
export const Route = createFileRoute("/login/")({
  component: RouteComponent,
});

function RouteComponent() {
  return <div>Hello "/login/"!</div>;
}
