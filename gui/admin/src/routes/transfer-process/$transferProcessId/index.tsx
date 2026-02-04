import { createFileRoute } from "@tanstack/react-router";
import dayjs from "dayjs";
import { useGetTransferProcessById } from "shared/src/data/transfer-queries.ts";
import { InfoList } from "shared/src/components/ui/info-list";
import { FormatDate } from "shared/src/components/ui/format-date";
import Heading from "shared/src/components/ui/heading.tsx";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "shared/src/components/ui/tabs";
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
import { Button } from "shared/src/components/ui/button";
import { TransferProcessActions } from "shared/src/components/actions/TransferProcessActions";
import TransferProcessMessageComponent from "shared/src/components/TransferProcessMessageComponent";
import { formatUrn } from "shared/src/lib/utils.ts";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageSection } from "shared/src/components/layout/PageSection";
import { InfoGrid } from "shared/src/components/layout/InfoGrid";

/**
 * Route for displaying individual transfer process details.
 */
export const Route = createFileRoute("/transfer-process/$transferProcessId/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { transferProcessId } = Route.useParams();
  const { data: transferProcess } = useGetTransferProcessById(transferProcessId);
  // const { data: dataPlane } = useGetDataplaneProcessById(transferProcessId);

  return (
    <PageLayout>
      <Tabs defaultValue="data-control" className="w-full">
        <TabsList>
          <TabsTrigger value="data-control">Control Plane</TabsTrigger>
        </TabsList>
        <TabsContent value="data-control">
          {" "}
          <PageSection title="Transfer process info">
            <InfoGrid className="mb-4">
              <InfoList
                items={[
                  { label: "Process pid", value: { type: "urn", value: transferProcess.id } },
                  { label: "Agreement id", value: { type: "urn", value: transferProcess.agreementId } },
                  {
                    label: "Transfer Process State",
                    value: { type: "status", value: transferProcess.state },
                  },
                  {
                    label: "Created at",
                    value: { type: "custom", content: <FormatDate date={transferProcess.createdAt} /> },
                  },
                  {
                    label: "Updated at",
                    value: { type: "custom", content: <FormatDate date={transferProcess.updatedAt} /> },
                  },
                ]}
              />
            </InfoGrid>
          </PageSection>

          <PageSection>
            {/* DRAWER */}
            <Drawer direction={"right"}>
              <DrawerTrigger>
                <Button variant={"secondary"}>See Transfer Process Messages</Button>
              </DrawerTrigger>
              <DrawerContent>
                <DrawerHeader>
                  <DrawerTitle>
                    <Heading level="h5" className="text-current">
                      Transfer Messages
                    </Heading>
                  </DrawerTitle>
                </DrawerHeader>
                {/* Messages */}
                <DrawerBody>
                  {transferProcess.messages.map((message) => {
                    return <TransferProcessMessageComponent key={message.id} message={message} />;
                  })}
                </DrawerBody>
                <DrawerFooter>
                  <DrawerClose>
                    <Button variant="ghost">Hide Messages</Button>
                  </DrawerClose>
                </DrawerFooter>
              </DrawerContent>
            </Drawer>
          </PageSection>
        </TabsContent>
      </Tabs>
      <TransferProcessActions process={transferProcess} tiny={false} />
    </PageLayout>
  );
}
