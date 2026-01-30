import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/gui/agreements/$agreement_id")({
  component: RouteComponent,
});

function RouteComponent() {
  const { agreement_id } = Route.useParams();
  return <div>hola {agreement_id}</div>;
}
