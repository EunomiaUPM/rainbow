import { createFileRoute, Link } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import { useGetParticipants } from "shared/src/data/participant-queries.ts";
import { DataTable } from "shared/src/components/DataTable";
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
import { Badge, BadgeRole } from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";

// Icons
import { ArrowRight, Plus } from "lucide-react";
import { useWalletOnboard } from "../../../../shared/src/data/wallet-mutations.ts";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext.tsx";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";

/**
 * Route for listing participants using a table layout.
 */
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
            {/* PARTICIPANTS TABLE */}
            <DataTable
              className="text-sm"
              data={participants ?? []}
              keyExtractor={(p) => p.participant_id}
              columns={[
                {
                  header: "Participant ID",
                  cell: (p) => <Badge variant={"info"}>{formatUrn(p.participant_id)}</Badge>,
                },
                {
                  header: "Identity Token",
                  cell: (p) => <Badge variant={"info"}>{formatUrn(p.token)}</Badge>,
                },
                {
                  header: "Participant Type",
                  cell: (p) => (
                    <Badge variant={"role"} role={p.participant_type as BadgeRole}>
                      {p.participant_type}
                    </Badge>
                  ),
                },
                {
                  header: "Base URL",
                  cell: (p) => <Badge variant={"info"}>{p.base_url}</Badge>,
                },
                {
                  header: "Link",
                  cell: (p) => (
                    <Link
                      to="/participants/$participantId"
                      params={{ participantId: p.participant_id }}
                    >
                      <Button variant="link">
                        See details
                        <ArrowRight />
                      </Button>
                    </Link>
                  ),
                },
              ]}
            />
            {/* /PARTICIPANTS TABLE */}
          </PageSection>
        </>
      )}
      {/* /WALLET OK */}
    </PageLayout>
  );
}
