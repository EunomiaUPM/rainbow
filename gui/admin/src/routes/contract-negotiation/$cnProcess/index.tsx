import { createFileRoute } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import dayjs from "dayjs";
import {
  useGetContractNegotiationMessagesByCNID,
  useGetContractNegotiationProcessesByCNID,
} from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/ContractNegotiationActions.tsx";
import { List, ListItem, ListItemDate, ListItemKey } from "shared/src/components/ui/list.tsx";
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
          <List>
            <ListItem>
              <ListItemKey>ProviderPid</ListItemKey>
              <Badge variant="info">{formatUrn(process.provider_id)}</Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>ConsumerPid</ListItemKey>
              <Badge variant="info">{formatUrn(process.consumer_id)}</Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>State</ListItemKey>
              <Badge variant="status" state={process.state as BadgeState}>
                {process.state}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>Client type</ListItemKey>
              <Badge>{process.is_business ? "Business" : "Standard"}</Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>Created at</ListItemKey>
              <ListItemDate>{dayjs(process.created_at).format("DD/MM/YYYY - HH:mm")}</ListItemDate>
            </ListItem>
          </List>
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
