import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/gui/agreements/hola")({
  component: RouteComponent,
});

function RouteComponent() {
  return <div>hola</div>;
}
