import { createFileRoute } from "@tanstack/react-router";
import dayjs from "dayjs";
import { useGetTransferProcessById } from "shared/src/data/transfer-queries.ts";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
import Heading from "shared/src/components/ui/heading.tsx";
import { Badge, BadgeState } from "shared/src/components/ui/badge.tsx";
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
          {/* <TabsTrigger value="data-plane">Data Plane</TabsTrigger> */}
        </TabsList>
        {/* <TabsContent value="data-plane" className="gridColsLayout">
          {dataPlane && (
            <div>
              <TransferProcessDataPlaneComponent dataPlane={dataPlane} />
            </div>
          )}
        </TabsContent> */}
        <TabsContent value="data-control">
          {" "}
          <PageSection title="Transfer process info">
            <InfoGrid className="mb-4">
              <List>
                <ListItem>
                  <ListItemKey>Process pid</ListItemKey>
                  <Badge variant={"info"}>{formatUrn(transferProcess.id)}</Badge>
                </ListItem>
                <ListItem>
                  <ListItemKey>Agreement id</ListItemKey>
                  <Badge variant={"info"}>{formatUrn(transferProcess.agreementId)}</Badge>
                </ListItem>
                <ListItem>
                  <ListItemKey>Transfer Process State</ListItemKey>
                  <Badge variant={"status"} state={transferProcess.state as BadgeState}>
                    {transferProcess.state}
                  </Badge>
                </ListItem>
                <ListItem>
                  <ListItemKey>Created at</ListItemKey>
                  <p> {dayjs(transferProcess.createdAt).format("DD/MM/YY HH:mm")}</p>
                </ListItem>
                <ListItem>
                  <ListItemKey>Updated at</ListItemKey>
                  <p> {dayjs(transferProcess.updatedAt).format("DD/MM/YY HH:mm")}</p>
                </ListItem>
              </List>
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
