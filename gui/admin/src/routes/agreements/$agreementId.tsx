import {createFileRoute} from "@tanstack/react-router";
import {useGetAgreementById, getAgreementByIdOptions} from "shared/src/data/agreement-queries";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import {List, ListItem, ListItemDate, ListItemKey} from "shared/src/components/ui/list";
import {Badge} from "shared/src/components/ui/badge.tsx";
import PolicyComponent from "shared/src/components/PolicyComponent.tsx";
import { formatUrn } from "shared/src/lib/utils.ts";

export const Route = createFileRoute("/agreements/$agreementId")({
  component: RouteComponent,
  loader: ({ context: { queryClient, api_gateway }, params: {agreementId} }) => {
      if (!api_gateway) return;
      return queryClient.ensureQueryData(getAgreementByIdOptions(api_gateway, agreementId));
  },
});

function RouteComponent() {
  const formatString = (text: string = "") => {
    return text.replace(/[()[]{}"]/g, " ");
  };
  const {agreementId} = Route.useParams();
  const {data: agreement} = useGetAgreementById(agreementId);
  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="mb-0.5 font-display flex gap-3 items-center">
        Agreement with id
        <Badge variant="info" size="lg">
          {" "}
          {formatUrn(agreement.agreement_id)}
        </Badge>
      </Heading>
      <div className="gridColsLayout">
        <div className=" ">
          <Heading level="h6" className="text-text">
            Agreement info
          </Heading>
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
        </div>
        <div>
          <Heading level="h6" className="text-text">
            Agreement content
          </Heading>
          {/* <div className="max-w-[940px]">{JSON.stringify(agreement.agreement_content)}</div> */}
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
                {/* {console.log(agreement.agreement_content.obligation, "")} */}
                <PolicyComponent
                  policyItem={agreement.agreement_content.prohibition}
                  variant={"prohibition"}
                />
              </div>
            </div>
          </List>
        </div>
      </div>
    </div>
  );
}
