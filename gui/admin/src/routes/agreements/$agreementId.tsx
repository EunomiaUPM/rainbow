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
        badge={<Badge variant="info" size="lg">{formatUrn(agreement.agreement_id)}</Badge>}
      />
      <InfoGrid>
        <PageSection title="Agreement info">
          <InfoList
            items={[
              { label: "Agreement Id", value: { type: "urn", value: agreement.agreement_id } },
              { label: "Related Message", value: { type: "urn", value: agreement.cn_message_id } },
              {
                label: "Consumer Participant Id",
                value: { type: "urn", value: agreement.consumer_participant_id },
              },
              {
                label: "Provider Participant Id",
                value: { type: "urn", value: agreement.provider_participant_id },
              },
              {
                label: "Status",
                value: {
                  type: "custom",
                  content: (
                    <Badge variant="status" state={agreement.active ? "ACTIVE" : "PAUSE"}>
                      {agreement.active ? "ACTIVE" : "INACTIVE"}
                    </Badge>
                  ),
                },
              },
              {
                label: "Created at",
                value: { type: "custom", content: <FormatDate date={agreement.created_at} /> },
              },
            ]}
          />
        </PageSection>
        <PageSection title="Agreement content">
          <InfoList
            items={[
              {
                label: "ID",
                value: {
                  type: "urn",
                  value: formatString(JSON.stringify(agreement.agreement_content["@id"])),
                },
              },
              {
                label: "Type",
                value: formatString(JSON.stringify(agreement.agreement_content["@type"])),
              },
              {
                label: "Assignee",
                value: {
                  type: "urn",
                  value: formatString(JSON.stringify(agreement.agreement_content.assignee)),
                },
              },
              {
                label: "Assigner",
                value: {
                  type: "urn",
                  value: formatString(JSON.stringify(agreement.agreement_content.assigner)),
                },
              },
              {
                label: "Policies",
                value: {
                  type: "custom",
                  content: (
                    <div className="flex flex-col gap-2 mb-2 w-full">
                      <PolicyComponent
                        policyItem={agreement.agreement_content.permission}
                        variant={"permission"}
                      />
                      <PolicyComponent
                        policyItem={agreement.agreement_content.obligation}
                        variant={"obligation"}
                      />

                      <PolicyComponent
                        policyItem={agreement.agreement_content.prohibition}
                        variant={"prohibition"}
                      />
                    </div>
                  ),
                },
              },
            ]}
          />
        </PageSection>
      </InfoGrid>
    </PageLayout>
  );
}
