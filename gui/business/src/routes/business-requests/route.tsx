import { createFileRoute, Outlet, redirect } from "@tanstack/react-router";

export const Route = createFileRoute("/business-requests")({
  component: RouteComponent,
  beforeLoad: ({ context }) => {
    // @ts-ignore
    if (!context.auth.isAuthenticated) {
      throw redirect({
        to: "/",
      });
    }
    // @ts-ignore
    if (context.auth.participant.participant_type == "Consumer") {
      throw redirect({
        to: "/customer-requests",
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
