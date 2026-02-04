import { createFileRoute, Link } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import dayjs from "dayjs";
import { DataTable } from "shared/src/components/DataTable";
import { FormatDate } from "shared/src/components/ui/format-date";
import { Button } from "shared/src/components/ui/button.tsx";
import { Badge, BadgeState } from "shared/src/components/ui/badge.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { useGetContractNegotiationProcesses } from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/actions/ContractNegotiationActions";
import { useMemo } from "react";
import { ArrowRight } from "lucide-react";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";

const RouteComponent = () => {
  const { data: cnProcesses } = useGetContractNegotiationProcesses();
  const cnProcessesSorted = useMemo(() => {
    if (!cnProcesses) return [];
    return [...cnProcesses].sort((a, b) => {
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });
  }, [cnProcesses]);

  return (
    <PageLayout>
      <PageHeader title="Contract Negotiations" />
      <PageSection>
        <div className=" pb-3 flex justify-between items-start">
          <div className=" basis-3/5">
            <Input type="search"></Input>
          </div>
        </div>
        <DataTable
          className="text-sm"
          data={cnProcessesSorted ?? []}
          keyExtractor={(p) => p.provider_id}
          columns={[
            {
              header: "ProviderPid",
              cell: (p) => <Badge variant={"info"}>{formatUrn(p.provider_id)}</Badge>,
            },
            {
              header: "ConsumerPid",
              cell: (p) => <Badge variant={"info"}>{formatUrn(p.consumer_id)}</Badge>,
            },
            {
              header: "State",
              cell: (p) => (
                <Badge variant={"status"} state={p.state as BadgeState}>
                  {p.state.replace("dspace:", "")}
                </Badge>
              ),
            },
            {
              header: "CreatedAt",
              cell: (p) => <FormatDate date={p.created_at} />,
            },
            {
              header: "Actions",
              cell: (p) => <ContractNegotiationActions process={p} tiny={true} />,
            },
            {
              header: "Link",
              cell: (p) => (
                <Link
                  to="/contract-negotiation/$cnProcess"
                  params={{ cnProcess: p.provider_id }}
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
};

/**
 * Route for listing contract negotiation processes.
 */
export const Route = createFileRoute("/contract-negotiation/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
