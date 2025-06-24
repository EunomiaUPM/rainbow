import { createFileRoute, Link } from "@tanstack/react-router";
import { useGetParticipants } from "shared/src/data/participant-queries.ts";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "shared/src/components/ui/drawer";
import { useContext } from "react";
import { PubSubContext } from "shared/src/context/PubSubContext.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { Badge } from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";

// Icons
import { ArrowRight, Plus } from "lucide-react";

export const Route = createFileRoute("/participants/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: participants } = useGetParticipants();
  const { lastHighLightedNotification } = useContext(PubSubContext)!;

  //     participants.map((participant) => {
  //     console.log(participant.participant_type);
  //   });

  return (
    <div>
      <div className="pb-3 w-full flex justify-between items-center">
        <div className="basis-3/5">
          <Input type="search"></Input>
        </div>
        {/* DRAWER ADD PARTICIPANT*/}
        <Drawer direction={"right"}>
          <DrawerTrigger>
            <Button>
              Add participant
              <Plus className="mb-1" />
            </Button>
          </DrawerTrigger>
          <DrawerContent>
            <DrawerHeader>
              <DrawerTitle>
                <Heading level="h5" className="text-current">
                  New Participant
                </Heading>
              </DrawerTitle>
            </DrawerHeader>
            {/* <NewParticipantForm/> */}
            <DrawerFooter>
              <DrawerClose className="flex justify-start gap-4">
                <Button variant="ghost" className="w-40">
                  Cancel
                </Button>
                {/* <Button className="w-40">Add Participant</Button> */}
              </DrawerClose>
            </DrawerFooter>
          </DrawerContent>
        </Drawer>
        {/* /DRAWER ADD PARTICIPANT*/}
      </div>

      <Table className="text-sm">
        <TableHeader>
          <TableRow>
            <TableHead>Participant ID</TableHead>
            <TableHead>Identity Token</TableHead>
            <TableHead>Participant Type</TableHead>
            <TableHead>Base URL</TableHead>
            {/* <TableHead>Extra Info</TableHead> */}
            <TableHead>Link</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {participants.map((participant) => (
            <TableRow
              key={participant.participant_id.slice(0, 20)}
              className={
                participant.participant_id === lastHighLightedNotification
                  ? "bg-blue-200"
                  : ""
              }
            >
              <TableCell>
                <Badge variant={"info"}>
                  {participant.participant_id.slice(9, 20) + "..."}
                </Badge>
              </TableCell>
              <TableCell>{participant.token?.slice(0, 20) + "..."}</TableCell>
              <TableCell>
                <Badge variant={"role"} role={participant.participant_type}>
                  {participant.participant_type}
                </Badge>
              </TableCell>
              <TableCell>
                <Badge variant={"info"}>{participant.base_url}</Badge>
              </TableCell>
              {/* <TableCell>{JSON.stringify(participant.extra_fields)}</TableCell> */}
              <TableCell>
                <Link
                  to="/participants/$participantId"
                  params={{ participantId: participant.participant_id }}
                >
                  <Button variant="link">
                    See details
                    <ArrowRight />
                  </Button>
                </Link>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
