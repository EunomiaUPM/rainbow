import { createFileRoute, Link } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import { useGetAgreements } from "shared/src/data/agreement-queries";
import { DataTable } from "shared/src/components/DataTable";
import { FormatDate } from "shared/src/components/ui/format-date";
import { Button } from "shared/src/components/ui/button.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { ArrowRight } from "lucide-react";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";

/**
 * Route for listing all agreements.
 */
export const Route = createFileRoute("/agreements/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: agreements } = useGetAgreements();


  return (
    <PageLayout>
      <PageHeader title="Agreements" />
      <PageSection>
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <DataTable
          className="text-sm"
          data={agreements ?? []}
          keyExtractor={(a) => a.agreement_id}
          columns={[
            {
              header: "Agreement Id",
              cell: (a) => <Badge variant={"info"}>{formatUrn(a.agreement_id)}</Badge>,
            },
            {
              header: "Consumer Participant Id",
              cell: (a) => (
                <div className="flex flex-col gap-1">
                  <Badge variant={"info"}>{formatUrn(a.consumer_participant_id)}</Badge>
                </div>
              ),
            },
            {
              header: "Status",
              cell: (a) => (
                <Badge variant={"status"} state={a.active ? "ACTIVE" : "PAUSE"}>
                  {a.active ? "ACTIVE" : "INACTIVE"}
                </Badge>
              ),
            },
            {
              header: "Created at",
              cell: (a) => <FormatDate date={a.created_at} />,
            },
            {
              header: "Link",
              cell: (a) => (
                <Link
                  to="/agreements/$agreementId"
                  params={{ agreementId: a.agreement_id }}
                >
                  <Button variant="link">
                    See details
                    <ArrowRight />
                  </Button>
                </Link>
              ),
            },
          ]}
        />
      </PageSection>
    </PageLayout>
  );
}
