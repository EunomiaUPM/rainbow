import { createFileRoute } from "@tanstack/react-router";
import { useGetContractNegotiationProcessesByCNID } from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/actions/ContractNegotiationActions";
import { InfoList } from "shared/src/components/ui/info-list";
import { FormatDate } from "shared/src/components/ui/format-date";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
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
import CnProcessMessageComponent from "@./../../shared/src/components/CnProcessMessageComponent.tsx";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageSection } from "shared/src/components/layout/PageSection";
import { InfoGrid } from "shared/src/components/layout/InfoGrid";

const RouteComponent = () => {
  const { cnProcess } = Route.useParams();
  const { data } = useGetContractNegotiationProcessesByCNID(cnProcess);
  const process = data as NegotiationProcessDto;

  return (
    <PageLayout>
      <InfoGrid className="mb-4">
        <PageSection title="Contract negotiation info">
          <InfoList
            items={[
              { label: "ProviderPid", value: { type: "urn", value: process.id } },
              { label: "State", value: { type: "status", value: process.state } },
              {
                label: "Created at",
                value: { type: "custom", content: <FormatDate date={process.createdAt} /> },
              },
            ]}
          />
        </PageSection>
      </InfoGrid>
      <PageSection>
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
              {process.messages.map((message) => (
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
