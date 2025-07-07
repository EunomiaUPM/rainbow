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
        <Badge
          variant={"info"}
          size={"lg"}
          className="max-w-[50%] truncate text-overflow-ellipsis"
        >
          {participant.participant_id.slice(9, -1)}
        </Badge>
      </Heading>
      {/* Page content */}
      <div className="gridColsLayout bg-blue-500/0">
        {/* Div Participant Info */}
        <div className="flex flex-col bg-green-800/0">
          <Heading
            level="h6"
            className="text-foreground h-[36px] place-content-center"
          >
            Participant info:
          </Heading>
          <div className="max-w-screen-md bg-green-500/0">
            <List className={"min-w-fit"}>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>
                  Participant ID
                </ListItemKey>
                <Badge variant={"info"}>
                  {participant.participant_id.slice(9, 29) + "[...]"}
                </Badge>
              </ListItem>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>
                  Identity Token
                </ListItemKey>
                <Badge variant={"info"}>{participant.token}</Badge>
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

        {/* Div Participant Tabs */}
        {agreements && agreements.length > 0 && (
          // Lo de arriba: evita que pete si no hay agreements que mostrar,
          // si se añaden otros tabs habría que añadirlos
          <Tabs defaultValue="participant-agreements">
            <TabsList>
              <TabsTrigger value="participant-agreements">
                Agreements
              </TabsTrigger>
              {/* <TabsTrigger value="participant-transferences">Transferences</TabsTrigger> */}
              {/* <TabsTrigger value="participant-contracts">Contract Negotiations</TabsTrigger> */}
            </TabsList>
            <TabsContent value="participant-agreements">
              <div className=" bg-pink-500/0">
                {/* <h2>Agreements</h2> */}
                <Table className="text-sm">
                  <TableHeader>
                    <TableRow>
                      <TableHead>Agreement Id</TableHead>
                      <TableHead className="hidden">Related Message</TableHead>
                      <TableHead className="hidden">
                        Consumer Participant Id
                      </TableHead>
                      <TableHead className="hidden">
                        Provider Participant Id
                      </TableHead>
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
                          </Badge>
                        </TableCell>
                        {/* <TableCell>
                  {agreement.cn_message_id?.slice(0, 20) + "..."}
                </TableCell className="hidden"> */}
                        {/* <TableCell>
                  <Badge variant={"info"}>
                    {agreement.consumer_participant_id?.slice(9, 20) + "..."}
                  </Badge>
                </TableCell> */}
                        {/* <TableCell className="hidden">
                          <Badge variant={"info"}>
                            {agreement.provider_participant_id?.slice(9, 20) +
                              "..."}
                          </Badge>
                        </TableCell> */}
                        <TableCell>
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
        )}
      </div>
    </div>
  );
}
