import { createFileRoute } from "@tanstack/react-router";
import { useGetAgreementById } from "@/data/agreement-queries.ts";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";

export const Route = createFileRoute("/agreements/$agreementId")({
  component: RouteComponent,
});


function RouteComponent() {
  const formatString = (text) =>{
    let formattedText = text.replace(/[()\[\]{}"]/g, ' ')
    return formattedText

  }
  const { agreementId } = Route.useParams();
  const { data: agreement } = useGetAgreementById(agreementId);
  return (
    <div className="space-y-4">
      <Heading level="h3" className="font-display">
        Agreement with id : {agreement.agreement_id}
      </Heading>
      <div>
        <Heading level="h6" className="text-text">
          Agreement info
        </Heading>
        <List>
          <ListItem>
            <ListItemKey>Agreement Id</ListItemKey>
            <TableCell>{agreement.agreement_id}</TableCell>
          </ListItem>
          <ListItem>
            <ListItemKey>Related Message</ListItemKey>
            <TableCell>{agreement.cn_message_id}</TableCell>
          </ListItem>
          <ListItem>
            <ListItemKey>Consumer Participant Id</ListItemKey>
            <TableCell>{agreement.consumer_participant_id}</TableCell>
          </ListItem>
          <ListItem>
            <ListItemKey>Provider Participant Id</ListItemKey>
            <TableCell>{agreement.provider_participant_id}</TableCell>
          </ListItem>
          <ListItem>
            <ListItemKey>Status</ListItemKey>
            <TableCell>{agreement.active ? "ACTIVE" : "INACTIVE"}</TableCell>
          </ListItem>
          <ListItem>
            <ListItemKey>CreatedAt</ListItemKey>
            <TableCell>
              {dayjs(agreement.created_at).format("DD/MM/YYYY - HH:mm")}
            </TableCell>
          </ListItem>
        </List>
      </div>
      <div>
        <Heading level="h6" className="text-text">
          Agreement content
        </Heading>
        {/* <div className="max-w-[940px]">{JSON.stringify(agreement.agreement_content)}</div> */}
      </div>
      <List>
        <ListItem>
          <ListItemKey> ID </ListItemKey>
          <p>{formatString(JSON.stringify(agreement.agreement_content["@id"]))}</p>
        </ListItem>
        <ListItem>
          <ListItemKey> Type </ListItemKey>
          <p>{formatString(JSON.stringify(agreement.agreement_content["@type"]))}</p>
        </ListItem>
        <ListItem>
          <ListItemKey> Assignee </ListItemKey>
          <p>{formatString(JSON.stringify(agreement.agreement_content.assignee))}</p>
        </ListItem>
        <ListItem>
          <ListItemKey> Assigner </ListItemKey>
          <p>{formatString(JSON.stringify(agreement.agreement_content.assigner))}</p>
        </ListItem>
        <div className="gap-1 flex flex-col">
          <ListItemKey className={" py-2 "}> Policies </ListItemKey>
          <div className="flex py-2 rounded-md border border-[#487a4b] bg-[#1e2422]">
            <p className="pl-3 w-1/2 font-bold uppercase text-[#a5e6b0] ">Permission</p>
            <p className="">
              {formatString(JSON.stringify(agreement.agreement_content.permission))}
            </p>
          </div>
          <div className="flex py-2 ">
            <p className="pl-3 w-1/2 font-bold">Obligation</p>
            <p className="">
              {formatString(JSON.stringify(agreement.agreement_content.obligation))}
            </p>
          </div>
          <div className="flex py-2 ">
            <p className="pl-3 w-1/2 font-bold">Prohibition</p>
            <p className="">
              {JSON.stringify(agreement.agreement_content.prohibition)}
            </p>
          </div>
        </div>
        {/* <ListItem>
                <ListItemKey> Obligation </ListItemKey>
                <p>{JSON.stringify(agreement.agreement_content.obligation)}</p>
            </ListItem>
             <ListItem>
                <ListItemKey> Prohibition </ListItemKey>
                <p>{JSON.stringify(agreement.agreement_content.prohibition)}</p>
            </ListItem> */}
      </List>
    </div>
  );
}
