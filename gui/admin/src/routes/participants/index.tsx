import { createFileRoute, Link } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
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
  DrawerBody,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "shared/src/components/ui/drawer";
import { useContext, useMemo } from "react";
import { PubSubContext } from "shared/src/context/PubSubContext.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { Badge } from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";

// Icons
import { ArrowRight, Plus } from "lucide-react";
import { useWalletOnboard } from "../../../../shared/src/data/wallet-mutations.ts";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext.tsx";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";

export const Route = createFileRoute("/participants/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: participants } = useGetParticipants();
  //const {lastHighLightedNotification} = useContext(PubSubContext)!;
  const { mutateAsync: onboardAsync } = useWalletOnboard();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

  const hasProvider = useMemo(() => {
    return (
      participants.find((p) => p.is_me == true && p.participant_type === "Agent") !== undefined
    );
  }, [participants]);

  const onboardHandler = async () => {
    await onboardAsync({
      api_gateway,
    });
  };

  return (
    <PageLayout>
      {/* NO WALLET */}
      {!hasProvider && (
        <div className="p-8 py-6 mx-auto w-fit max-w-[70ch] bg-brand-sky/5 border border-stroke rounded-md">
          <Heading level="h3">Missing wallet...</Heading>
          <Heading level="h5">
            Your wallet is not yet connected as Provider. <br /> Please complete the onboarding
            process to get started.
          </Heading>
          <Button size={"lg"} className="w-full mt-4" onClick={() => onboardHandler()}>
            Onboard wallet
          </Button>
        </div>
      )}
      {/* TO DO - loading screen */}
      {/* /NO WALLET */}

      {/* WALLET OK */}
      {hasProvider && (
        <>
          <PageHeader title="Participants" />
          <PageSection>
            {/* HEADER CONTAINER */}
            <div className="pb-3 w-full flex justify-between items-center">
              <div className="basis-3/5">
                <Input type="search"></Input>
              </div>

              {/* DRAWER ADD PARTICIPANT*/}
              {/** TO DO: DELETE */}
              <Drawer direction={"right"}>
                <DrawerTrigger className="hidden">
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
                  <DrawerBody>{/* <NewParticipantForm/> */}</DrawerBody>
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
              {/** TO DO: DELETE */}
            </div>
            {/* /HEADER CONTAINER */}

            {/* PARTICIPANTS TABLE */}
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
                  <TableRow key={formatUrn(participant.participant_id)}>
                    <TableCell>
                      <Badge variant={"info"}>
                        {formatUrn(participant.participant_id)}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <Badge variant={"info"}>{formatUrn(participant.token)}</Badge>
                    </TableCell>
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
            {/* /PARTICIPANTS TABLE */}
          </PageSection>
        </>
      )}
      {/* /WALLET OK */}
    </PageLayout>
  );
}
