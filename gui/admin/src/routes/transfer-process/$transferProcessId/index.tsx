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
import { TransferProcessActions } from "shared/src/components/TransferProcessActions";
import TransferProcessMessageComponent from "shared/src/components/TransferProcessMessageComponent";

export const Route = createFileRoute("/transfer-process/$transferProcessId/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { transferProcessId } = Route.useParams();
  const { data: transferProcess } = useGetTransferProcessById(transferProcessId);
  // const { data: dataPlane } = useGetDataplaneProcessById(transferProcessId);

  return (
    <div className="space-y-4 pb-4">
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
          <Heading level="h5" className="mt-3">
            Transfer process info{" "}
          </Heading>
          <div className="mb-4 gridColsLayout">
            <List>
              <ListItem>
                <ListItemKey>Process pid</ListItemKey>
                <Badge variant={"info"}>{transferProcess.id.slice(9, 20) + "..."}</Badge>
              </ListItem>
              <ListItem>
                <ListItemKey>Agreement id</ListItemKey>
                <Badge variant={"info"}>{transferProcess.agreementId.slice(9, 20) + "..."}</Badge>
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
          </div>
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
        </TabsContent>
      </Tabs>
      <TransferProcessActions process={transferProcess} tiny={false} />
    </div>
  );
}
