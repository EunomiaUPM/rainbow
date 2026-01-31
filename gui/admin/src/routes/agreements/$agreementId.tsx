import { createFileRoute } from "@tanstack/react-router";
import { useGetAgreementById, getAgreementByIdOptions } from "shared/src/data/agreement-queries";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import { List, ListItem, ListItemDate, ListItemKey } from "shared/src/components/ui/list";
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
          <List>
            <ListItem>
              <ListItemKey>Agreement Id</ListItemKey>
              <Badge variant="info">{formatUrn(agreement.agreement_id)}</Badge>
            </ListItem>
            <div className={"border-b border-white/20"}>
              <ListItem>
                <ListItemKey>Related Message</ListItemKey>
                <Badge variant="info">{formatUrn(agreement.cn_message_id)}</Badge>
              </ListItem>
            </div>
            <ListItem>
              <ListItemKey>Consumer Participant Id</ListItemKey>
              <Badge variant="info">
                {formatUrn(agreement.consumer_participant_id)}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>Provider Participant Id</ListItemKey>{" "}
              <Badge variant="info">
                {formatUrn(agreement.provider_participant_id)}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>Status</ListItemKey>
              <Badge variant="status" state={agreement.active ? "ACTIVE" : "PAUSE"}>
                {agreement.active ? "ACTIVE" : "INACTIVE"}{" "}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>Created at</ListItemKey>
              <ListItemDate>
                {dayjs(agreement.created_at).format("DD/MM/YYYY - HH:mm")}
              </ListItemDate>
            </ListItem>
          </List>
        </PageSection>
        <PageSection title="Agreement content">
          <List>
            <ListItem>
              <ListItemKey> ID </ListItemKey>
              <Badge variant="info">
                {formatString(
                  JSON.stringify(formatUrn(agreement.agreement_content["@id"])),
                )}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey> Type </ListItemKey>
              <p>{formatString(JSON.stringify(agreement.agreement_content["@type"]))}</p>
            </ListItem>
            <ListItem>
              <ListItemKey> Assignee </ListItemKey>
              <Badge variant="info">
                {formatString(
                  JSON.stringify(formatUrn(agreement.agreement_content.assignee)),
                )}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey> Assigner </ListItemKey>
              <Badge variant="info">
                {formatString(
                  JSON.stringify(formatUrn(agreement.agreement_content.assigner)),
                )}
              </Badge>
            </ListItem>

            <div className="gap-1 flex flex-col">
              <ListItemKey className={" py-2 "}>Policies</ListItemKey>

              <div className="flex flex-col gap-2 mb-2">
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
            </div>
          </List>
        </PageSection>
      </InfoGrid>
    </PageLayout>
  );
}
