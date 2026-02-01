import { createFileRoute } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import dayjs from "dayjs";
import {
  useGetContractNegotiationMessagesByCNID,
  useGetContractNegotiationProcessesByCNID,
} from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/actions/ContractNegotiationActions";
import { InfoList } from "shared/src/components/ui/info-list";
import { FormatDate } from "shared/src/components/ui/format-date";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import { Badge, BadgeState } from "shared/src/components/ui/badge.tsx";
import {
  Drawer,
  DrawerBody,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "@./../../shared/src/components/ui/drawer.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { useEffect } from "react";
import CnProcessMessageComponent
  from "@./../../shared/src/components/CnProcessMessageComponent.tsx";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageSection } from "shared/src/components/layout/PageSection";
import { InfoGrid } from "shared/src/components/layout/InfoGrid";

const RouteComponent = () => {

  const { cnProcess } = Route.useParams();
  const { data } = useGetContractNegotiationProcessesByCNID(cnProcess);
  const process = data as CNProcess;
  const { data: cnMessages } = useGetContractNegotiationMessagesByCNID(cnProcess);

  return (
    <PageLayout>
      <PageSection title="Contract negotiation info">
        <InfoGrid className="mb-4">
          <InfoList
            items={[
              { label: "ProviderPid", value: { type: "urn", value: process.provider_id } },
              { label: "ConsumerPid", value: { type: "urn", value: process.consumer_id } },
              { label: "State", value: { type: "status", value: process.state } },
              {
                label: "Client type",
                value: { type: "custom", content: <Badge>{process.is_business ? "Business" : "Standard"}</Badge> },
              },
              {
                label: "Created at",
                value: { type: "custom", content: <FormatDate date={process.created_at} /> },
              },
            ]}
          />
        </InfoGrid>
      </PageSection>
      <PageSection>
        {/*  */}

        {/* DRAWER */}
        <Drawer direction={"right"}>
          <DrawerTrigger>
            <Button variant={"secondary"}>See Contract Negotiation Messages</Button>
          </DrawerTrigger>
          <DrawerContent>
            <DrawerHeader>
              <DrawerTitle>
                <Heading level="h5" className="text-current">
                  Contract negotiation Messages
                </Heading>
              </DrawerTitle>
            </DrawerHeader>
            <DrawerBody>
              {/* New message subcomponent */}
              {cnMessages.map((message) => (
                <CnProcessMessageComponent message={message} />
              ))}
              {/* / New message subcomponent */}
            </DrawerBody>
            <DrawerFooter>
              <DrawerClose>
                <Button variant="ghost">Hide Messages</Button>
              </DrawerClose>
            </DrawerFooter>
          </DrawerContent>
        </Drawer>
      </PageSection>

      {/* ACTIONS */}
      <ContractNegotiationActions process={process} tiny={false} />
    </PageLayout>
  );
};

/**
 * Route for displaying contract negotiation process details.
 */
export const Route = createFileRoute("/contract-negotiation/$cnProcess/")({
  component: RouteComponent,
});
