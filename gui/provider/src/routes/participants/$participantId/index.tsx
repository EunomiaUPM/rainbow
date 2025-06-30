import { createFileRoute, Link } from "@tanstack/react-router";
import {
  useGetAgreementsByParticipantId,
  useGetParticipantById,
} from "shared/src/data/participant-queries.ts";
import dayjs from "dayjs";

// Components
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "../../../../../shared/src/components/ui/tabs.tsx";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
// Icons
import { ArrowRight } from "lucide-react";
import { Separator } from "shared/src/components/ui/separator.tsx";

export const Route = createFileRoute("/participants/$participantId/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { participantId } = Route.useParams();
  const { data: participant } = useGetParticipantById(participantId);
  const { data: agreements } = useGetAgreementsByParticipantId(participantId);

  const scopedListItemKeyClasses = "basis-[28%]";

  return (
    <div className="w-full">
      {/* Page Header */}
      <Heading level="h3" className="flex items-center gap-3">
        Participant with id
        <Badge variant={"info"} size={"lg"}>
          {participant.participant_id.slice(9, -1)}
        </Badge>
      </Heading>
      {/* Page content */}
      <div className="flexColsLayout bg-blue-500/0">
        {/* Div Participant Info */}
        <div className="flex flex-col bg-green-800/0">
          <Heading
            level="h6"
            className="text-foreground h-[36px] place-content-center"
          >
            {" "}
            Participant info:{" "}
          </Heading>
          <div className="w-full bg-green-500/0">
            <List className={" xl:w-[400px] 2xl:min-w-fit"}>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>
                  Participant ID
                </ListItemKey>
                <Badge variant={"info"} className="grow-1">
                  {participant.participant_id.slice(9, -1) + "[...]"}
                </Badge>
              </ListItem>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>
                  Identity Token
                </ListItemKey>
                {participant.token}
              </ListItem>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>
                  Participant Type
                </ListItemKey>
                <Badge variant={"role"} role={participant.participant_type}>
                  {participant.participant_type}
                </Badge>
              </ListItem>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>
                  Base URL
                </ListItemKey>
                <Badge variant={"info"}>{participant.base_url}</Badge>
              </ListItem>
            </List>
          </div>
        </div>
        <Separator orientation="vertical" />

        {/* Div Participant Tabs */}
        <Tabs defaultValue="participant-agreements">
          <TabsList>
            <TabsTrigger value="participant-agreements">Agreements</TabsTrigger>
            {/* <TabsTrigger value="participant-transferences">Transferences</TabsTrigger> */}
            {/* <TabsTrigger value="participant-contracts">Contract Negotiations</TabsTrigger> */}
          </TabsList>
          <TabsContent value="participant-agreements">
            <div className=" bg-pink-500/0">
              {/* <h2>Agreements</h2> */}
              <Table className="text-sm">
                <TableHeader>
                  <TableRow>
                    <TableHead>Agreement ID</TableHead>
                    {/* <TableHead>Related Message</TableHead> */}
                    {/* <TableHead>Consumer Participant Id</TableHead> */}
                    {/* <TableHead>Provider Participant Id</TableHead> */}
                    <TableHead>Status</TableHead>
                    <TableHead>Created at</TableHead>
                    <TableHead>Link</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {agreements.map((agreement) => (
                    <TableRow key={agreement.agreement_id.slice(0, 20)}>
                      <TableCell className="!w-fit !max-w-fit">
                        <Badge variant={"info"}>
                          {agreement.agreement_id.slice(9, 20) + "..."}{" "}
                          {/* Desde 9 -> Sólo número */}
                        </Badge>
                      </TableCell>
                      {/* <TableCell>
                  {agreement.cn_message_id?.slice(0, 20) + "..."}
                </TableCell> */}
                      {/* <TableCell>
                  <Badge variant={"info"}>
                    {agreement.consumer_participant_id?.slice(9, 20) + "..."}
                  </Badge>
                </TableCell> */}
                      {/* <TableCell>
                  <Badge variant={"info"}>
                    {agreement.provider_participant_id?.slice(9, 20) + "..."}
                  </Badge>
                </TableCell> */}
                      <TableCell>
                        {/* {console.log(agreements, "agreements en provider")} */}
                        <Badge
                          variant={"status"}
                          state={agreement.active ? "STARTED" : "PAUSE"}
                        >
                          {agreement.active ? "ACTIVE" : "INACTIVE"}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        {dayjs(agreement.created_at).format("DD/MM/YY HH:mm")}
                      </TableCell>
                      <TableCell>
                        <Link
                          to="/agreements/$agreementId"
                          params={{ agreementId: agreement.agreement_id }}
                        >
                          <Button variant="link">
                            See agreement
                            <ArrowRight />
                          </Button>
                        </Link>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  );
}
