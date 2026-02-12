import { createFileRoute } from "@tanstack/react-router";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import { InfoList } from "shared/src/components/ui/info-list";
import { FormatDate } from "shared/src/components/ui/format-date";
import {
  useGetConnectorInstanceByDistribution,
  getGetConnectorInstanceByDistributionQueryOptions,
} from "shared/src/data/orval/connector/connector";
import {
  useGetDistributionById,
  getGetDistributionByIdQueryOptions,
} from "shared/src/data/orval/distributions/distributions";
import { formatUrn } from "shared/src/lib/utils.ts";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";

function RouteComponent() {
  const { distributionId } = Route.useParams();
  const { data: distributionData } = useGetDistributionById(distributionId);
  const { data: connectorData } = useGetConnectorInstanceByDistribution(distributionId);

  const distribution = distributionData?.status === 200 ? distributionData.data : undefined;
  const connector = connectorData?.status === 200 ? connectorData.data : undefined;

  return (
    <PageLayout>
      <PageHeader
        title="Distribution info with id"
        badge={
          <Badge variant="info" size="lg">
            {formatUrn(distribution?.id!)}
          </Badge>
        }
      />
      <PageSection>
        <InfoList
          className="text-sm"
          items={[
            { label: "Distribution title", value: distribution?.dctTitle },
            {
              label: "Distribution creation date",
              value: { type: "custom", content: <FormatDate date={distribution?.dctIssued!} /> },
            },
          ]}
        />
      </PageSection>
      <PageSection title="Connector Instance info">
        <InfoList
          className="text-sm"
          items={[
            {
              label: "Connector Instance info",
              value: { type: "custom", content: <pre>{JSON.stringify(connector, null, 2)}</pre> },
            },
          ]}
        />
      </PageSection>
    </PageLayout>
  );
}

/**
 * Route for displaying distribution connector details.
 */
export const Route = createFileRoute("/catalog/$catalogId/distribution-connector/$distributionId")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
  loader: async ({ context: { queryClient }, params: { distributionId } }) => {
    await queryClient.ensureQueryData(getGetDistributionByIdQueryOptions(distributionId));
    return queryClient.ensureQueryData(getGetConnectorInstanceByDistributionQueryOptions(distributionId));
  },
});
