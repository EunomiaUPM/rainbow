import { createFileRoute, Outlet, redirect } from "@tanstack/react-router";

export const Route = createFileRoute("/customer-requests")({
  component: RouteComponent,
  beforeLoad: ({ context }) => {
    // @ts-ignore
    if (!context.auth.isAuthenticated) {
      throw redirect({
        to: "/",
      });
    }
    // @ts-ignore
    if (context.auth.participant.participant_type == "Provider") {
      throw redirect({
        to: "/business-requests",
      });
    }
  },
});

function RouteComponent() {
  return (
    <div>
      <Outlet />
    </div>
  );
}
