import { createFileRoute, Outlet } from "@tanstack/react-router";
import { RouteTabs } from "shared/src/components/layout/RouteTabs";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";

const tabs = [
  { label: "Sent", to: "/connections/sent" },
  { label: "Received", to: "/connections/received" },
];

const ConnectionsLayout = () => {
  return (
    <PageLayout>
      <PageHeader title="My Connections" />

      <RouteTabs tabs={tabs} />
      <Outlet />
    </PageLayout>
  );
};

export const Route = createFileRoute("/connections")({
  component: ConnectionsLayout,
});
