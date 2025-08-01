import { createFileRoute } from "@tanstack/react-router";
import { useGetAgreementById } from "shared/src/data/agreement-queries";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import { List, ListItem, ListItemDate, ListItemKey } from "shared/src/components/ui/list.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import PolicyComponent from "shared/src/components/PolicyComponent.tsx";

export const Route = createFileRoute("/agreements/$agreementId")({
  component: RouteComponent,
});

function RouteComponent() {
  const formatString = (text: string) => {
    let formattedText = text.replace(/[()\[\]{}"]/g, " ");
    return formattedText;
  };
  const { agreementId } = Route.useParams();
  const { data: agreement } = useGetAgreementById(agreementId);

  const scopedListItemKeyClasses = " "; // basis-[33%] min-w-[100px]
  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="mb-0.5 font-display flex gap-3 items-center">
        Agreement with id
        <Badge variant="info" size="lg">
          {" "}
          {agreement.agreement_id.slice(9, 29) + "[...]"}
        </Badge>
      </Heading>
      <div className="gridColsLayout">
        <div>
          <Heading level="h6" className="text-text">
            Agreement info
          </Heading>
          <List>
            <ListItem>
              <ListItemKey className={scopedListItemKeyClasses}>Agreement Id</ListItemKey>
              <Badge variant="info"> {agreement.agreement_id.slice(9, 29) + "[...]"}</Badge>
            </ListItem>
            <ListItem>
              <ListItemKey className={scopedListItemKeyClasses}>
                Provider Participant id:
              </ListItemKey>
              <Badge variant={"info"}>{agreement.agreement_id.slice(9, 29) + "[...]"}</Badge>
            </ListItem>
            <div className={"border-b border-white/20"}>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>Related Message</ListItemKey>
                <Badge variant="info">{agreement.cn_message_id.slice(9, 29) + "[...]"}</Badge>
              </ListItem>
            </div>
            <ListItem>
              <ListItemKey className={scopedListItemKeyClasses}>
                Consumer Participant Id
              </ListItemKey>
              <Badge variant="info">
                {agreement.consumer_participant_id.slice(9, 29) + "[...]"}{" "}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey className={scopedListItemKeyClasses}>
                Provider Participant Id
              </ListItemKey>{" "}
              <Badge variant="info">
                {agreement.provider_participant_id.slice(9, 29) + "[...]"}{" "}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey className={scopedListItemKeyClasses}>Status</ListItemKey>{" "}
              <Badge variant="status" state={agreement.active ? "ACTIVE" : "PAUSE"}>
                {agreement.active ? "ACTIVE" : "INACTIVE"}{" "}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey className={scopedListItemKeyClasses}>Created at</ListItemKey>
              <ListItemDate>
                {dayjs(agreement.created_at).format("DD/MM/YYYY - HH:mm")}{" "}
              </ListItemDate>
            </ListItem>
          </List>
        </div>

        <div>
          <Heading level="h6" className="text-text">
            Agreement content
          </Heading>
          <List>
            <ListItem>
              <ListItemKey> ID </ListItemKey>
              <Badge variant="info">
                {formatString(
                  JSON.stringify(agreement.agreement_content["@id"].slice(9, 29) + "[...]"),
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
                  JSON.stringify(agreement.agreement_content.assignee.slice(9, 29) + "[...]"),
                )}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey> Assigner </ListItemKey>
              <Badge variant="info">
                {formatString(
                  JSON.stringify(agreement.agreement_content.assigner.slice(9, 29) + "[...]"),
                )}
              </Badge>
            </ListItem>

            <div className="gap-1 flex flex-col">
              <ListItemKey className={" py-2 "}> Policies </ListItemKey>

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
        </div>
      </div>
    </div>
  );
}
