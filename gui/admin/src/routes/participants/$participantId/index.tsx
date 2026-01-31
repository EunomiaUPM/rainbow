import { createFileRoute, Link } from "@tanstack/react-router";
import {
  useGetAgreementsByParticipantId,
  useGetParticipantById,
} from "shared/src/data/participant-queries.ts";
import { formatUrn } from "shared/src/lib/utils.ts";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";
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
import { Badge, BadgeRole } from "shared/src/components/ui/badge.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
// Icons
import { ArrowRight } from "lucide-react";

export const Route = createFileRoute("/participants/$participantId/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { participantId } = Route.useParams();
  const { data: participant } = useGetParticipantById(participantId);
  //const {data: agreements} = useGetAgreementsByParticipantId(participantId);

  const scopedListItemKeyClasses = "basis-[28%]";

  return (
    <PageLayout>
      {/* Page Header */}
      <PageHeader
        title="Participant with id"
        badge={
          <Badge variant={"info"} size={"lg"} className="max-w-[50%] truncate text-overflow-ellipsis">
            {formatUrn(participant.participant_id)}
          </Badge>
        }
      />
      {/* Page content */}
      <div className="bg-blue-500/0">
        {/* Div Participant Info */}
        <PageSection title="Participant info:">
          <div className="max-w-screen-md bg-green-500/0">
            <List className={"min-w-fit"}>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>Participant ID</ListItemKey>
                <Badge variant={"info"}>{formatUrn(participant.participant_id)}</Badge>
              </ListItem>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>Identity Token</ListItemKey>
                <Badge variant={"info"}>{participant.token}</Badge>
              </ListItem>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>Participant Type</ListItemKey>
                <Badge variant={"role"} dsrole={participant.participant_type as BadgeRole}>
                  {participant.participant_type}
                </Badge>
              </ListItem>
              <ListItem>
                <ListItemKey className={scopedListItemKeyClasses}>Base URL</ListItemKey>
                <Badge variant={"info"}>{participant.base_url}</Badge>
              </ListItem>
            </List>
          </div>
        </PageSection>

        {/* Div Participant Tabs */}
        {/* {agreements && agreements.length > 0 && (
          <Tabs defaultValue="participant-agreements">
            <TabsList>
              <TabsTrigger value="participant-agreements">Agreements</TabsTrigger>
            </TabsList>
            <TabsContent value="participant-agreements">
              <div className=" bg-pink-500/0">
                <h2>Agreements</h2>
                <Table className="text-sm">
                  <TableHeader>
                    <TableRow>
                      <TableHead>Agreement ID</TableHead>
                      <TableHead className="hidden">Related Message</TableHead>
                      <TableHead className="hidden">Consumer Participant Id</TableHead>
                      <TableHead className="hidden">Provider Participant Id</TableHead>
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

                        <TableCell>
                          <Badge variant={"status"} state={agreement.active ? "STARTED" : "PAUSE"}>
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
        )} */}
        <div>
          <Heading level="h5">Agreements to be done...</Heading>
        </div>
      </div>
    </PageLayout>
  );
}
