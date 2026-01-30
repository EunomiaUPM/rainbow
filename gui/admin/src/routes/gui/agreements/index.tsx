import { createFileRoute, Link } from "@tanstack/react-router";

export const Route = createFileRoute("/gui/agreements/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div>
      alsdaslkdnalskdnalksn
      <Link to="/gui/agreements/hola">Hola</Link>
      <Link
        to="/gui/agreements/$agreement_id"
        params={{
          agreement_id: "123",
        }}
      >
        quiero a agreement 123
      </Link>
    </div>
  );
}
