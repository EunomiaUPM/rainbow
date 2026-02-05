import { createFileRoute } from "@tanstack/react-router";
import { useGetAgreementById, getAgreementByIdOptions } from "shared/src/data/agreement-queries";
import { InfoList } from "shared/src/components/ui/info-list";
import { FormatDate } from "shared/src/components/ui/format-date";
import { Badge } from "shared/src/components/ui/badge.tsx";
import PolicyComponent from "shared/src/components/PolicyComponent.tsx";
import { formatUrn } from "shared/src/lib/utils.ts";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";
import { InfoGrid } from "shared/src/components/layout/InfoGrid";
import { PolicyWrapperShow } from "shared/components/PolicyWrapperShow";

/**
 * Route for displaying agreement details.
 */
export const Route = createFileRoute("/agreements/$agreementId")({
  component: RouteComponent,
  loader: ({ context: { queryClient, api_gateway }, params: { agreementId } }) => {
    if (!api_gateway) return;
    return queryClient.ensureQueryData(getAgreementByIdOptions(api_gateway, agreementId));
  },
});

function RouteComponent() {
  const formatString = (text: string = "") => {
    return text.replace(/[()[]{}"]/g, " ");
  };
  const { agreementId } = Route.useParams();
  const { data: agreement } = useGetAgreementById(agreementId);
  return (
    <PageLayout>
      <PageHeader
        title="Agreement with id"
        badge={
          <Badge variant="info" size="lg">
            {formatUrn(agreement.id)}
          </Badge>
        }
      />
      <InfoGrid>
        <PageSection title="Agreement info">
          <InfoList
            items={[
              { label: "Agreement Id", value: { type: "urn", value: agreement.id } },
              {
                label: "Related Message",
                value: { type: "urn", value: agreement.negotiationAgentMessageId },
              },
              {
                label: "Consumer Participant Id",
                value: { type: "urn", value: agreement.consumerParticipantId },
              },
              {
                label: "Provider Participant Id",
                value: { type: "urn", value: agreement.providerParticipantId },
              },
              {
                label: "Status",
                value: {
                  type: "custom",
                  content: (
                    <Badge variant="status" state={agreement.state ? "ACTIVE" : "PAUSE"}>
                      {agreement.state}
                    </Badge>
                  ),
                },
              },
              {
                label: "Created at",
                value: { type: "custom", content: <FormatDate date={agreement.createdAt} /> },
              },
            ]}
          />
        </PageSection>

        <PolicyWrapperShow
          key={agreement.agreementContent["@id"]}
          policy={agreement.agreementContent}
          participant={{ participant_type: "Consumer" }}
          datasetId={agreement.agreementContent.target}
          catalogId={undefined}
          datasetName={agreement.agreementContent.target}
        />


      </InfoGrid>
    </PageLayout>
  );
}
