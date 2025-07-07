import {createFileRoute} from "@tanstack/react-router";
import dayjs from "dayjs";
import {
  useGetDataplaneProcessById,
  useGetTransferMessagesByProviderPid,
  useGetTransferProcessByProviderPid,
} from "shared/src/data/transfer-queries.ts";
import {List, ListItem, ListItemKey} from "shared/src/components/ui/list.tsx";
import Heading from "shared/src/components/ui/heading.tsx";
import {Badge} from "shared/src/components/ui/badge.tsx";
import {Tabs, TabsContent, TabsList, TabsTrigger,} from "../../../../../shared/src/components/ui/tabs.tsx";
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
import {Button} from "shared/src/components/ui/button.tsx";
import {TransferProcessActions} from "shared/src/components/TransferProcessActions.tsx";
import TransferProcessDataPlaneComponent from "shared/src/components/TransferProcessDataPlaneComponent.tsx";
import TransferProcessMessageComponent from "shared/src/components/TransferProcessMessageComponent.tsx";

export const Route = createFileRoute("/transfer-process/$transferProcessId/")({
    component: RouteComponent,
});

function RouteComponent() {

    const {transferProcessId} = Route.useParams();
    const {data: transferProcess} = useGetTransferProcessByProviderPid(transferProcessId);
    const {data: transferMessages} = useGetTransferMessagesByProviderPid(transferProcessId);
    const {data: dataPlane} = useGetDataplaneProcessById(transferProcess.consumer_pid);

    return (
        <div className="space-y-4 pb-4">
            <Tabs defaultValue="data-control" className="w-full">
                <TabsList>
                    <TabsTrigger value="data-control">Data Control</TabsTrigger>
                    <TabsTrigger value="data-plane">Data Plane</TabsTrigger>
                </TabsList>
                <TabsContent value="data-plane" className="gridColsLayout">
                    {/* <div className=" w-full break-all">
            <pre className="max-w-[500px] p-4 rounded-lg break-all text-[11px] bg-black/70 text-secondary-400 break-all">
              <code className="whitespace-pre-wrap break-all">
                {JSON.stringify(dataPlane, null, 2)}
              </code>
            </pre>
          </div> */}
                    {dataPlane && (
                        <div>
                            <TransferProcessDataPlaneComponent dataPlane={dataPlane}/>
                        </div>
                    )}
                </TabsContent>
                <TabsContent value="data-control">
                    {" "}
                    <Heading level="h5" className="mt-3">
                        Transfer process info{" "}
                    </Heading>
                    <div className="mb-4 gridColsLayout">
                        <List>
                            <ListItem>
                                <ListItemKey>Process pid</ListItemKey>
                                <Badge variant={"info"}>{transferProcess.provider_pid.slice(9, 20) + "..."}</Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Consumer pid</ListItemKey>
                                <Badge variant={"info"}>{transferProcess.consumer_pid.slice(9, 20) + "..."}</Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Transfer Process State</ListItemKey>
                                <Badge variant={"status"} state={transferProcess.state}>
                                    {transferProcess.state}
                                </Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Created at</ListItemKey>
                                <p> {dayjs(transferProcess.created_at).format("DD/MM/YY HH:mm")}</p>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Updated at</ListItemKey>
                                <p> {dayjs(transferProcess.updated_at).format("DD/MM/YY HH:mm")}</p>
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
                                {transferMessages.map((message) => {
                                    // console.log(message);
                                    return (
                                        // <pre key={message.id}>{JSON.stringify(message, null, 2)}</pre>
                                        <TransferProcessMessageComponent key={message.id} message={message}/>
                                    );
                                })}
                            </DrawerBody>
                            {/* esto por si hace falta en algn momento dividir lo que pone en el
          content, pero por ahora no hace falta!!
          <div className="flex ">
            <p className="font-bold min-w-[9.4rem] "> Transfer Content: </p>
            <p> {transferMessage.content["@context"]} </p>
          </div>
          <div> {transferMessage.content["@type"]} </div>
          <div> {transferMessage.content.agreementId} </div>
          <div className="flex ">
            <p className="font-bold min-w-[9.4rem] "> Agreement ID </p>
            <Badge variant="info">
              {" "}
              {transferMessage.content["@context"]}{" "}
            </Badge>
          </div> */}

                            <DrawerFooter>
                                <DrawerClose>
                                    <Button variant="ghost">Hide Messages</Button>
                                </DrawerClose>
                            </DrawerFooter>
                        </DrawerContent>
                    </Drawer>
                </TabsContent>
            </Tabs>

            <div>
                {/* CATALOGOS CON LAS VARIABLES, NO BORRAR!! */}
                {/* <Table className="text-sm">
           <TableHeader>
                    <TableRow>
                        <TableHead>Key</TableHead>
                        <TableHead>Value</TableHead>
                    </TableRow>
                </TableHeader>
          <TableBody> */}
                {/* <TableRow>
                        <TableCell>Transfer Process Provider pid</TableCell>
                        <TableCell>{transferProcess.provider_pid.slice(0, 20) + "..."}</TableCell>
                    </TableRow> */}
                {/* <TableRow>
                        <TableCell>Transfer Consumer Provider pid</TableCell>
                        <TableCell>{transferProcess.consumer_pid.slice(0, 20) + "..."}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Agreement id</TableCell>
                        <TableCell>{transferProcess.agreement_id.slice(0, 20) + "..."}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>State</TableCell>
                        <TableCell>{transferProcess.state}</TableCell>
                    </TableRow> */}
                {/* <TableRow>
              <TableCell>Created At</TableCell>
              <TableCell>
                {dayjs(transferProcess.created_at).format("DD/MM/YYYY - HH:mm")}
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>Updated At</TableCell>
              <TableCell>
                {dayjs(transferProcess.updated_at).format("DD/MM/YYYY - HH:mm")}
              </TableCell>
            </TableRow> */}
                {/* </TableBody>
        </Table> */}
            </div>

            <div>
                {/* MENSAJES */}
                {/* <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Transfer Message Id</TableHead>
              <TableHead>Transfer Process id</TableHead>
              <TableHead>Message type</TableHead>
              <TableHead>Created At</TableHead>
              <TableHead>From</TableHead>
              <TableHead>To</TableHead>
              <TableHead>Content</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {transferMessages.map((transferMessage) => (
              <TableRow key={transferMessage.id.slice(0, 20)}>
                <TableCell>
                  <Link
                    to="/transfer-process/$transferProcessId/transfer-message/$transferMessageId"
                    params={{
                      transferProcessId: transferProcessId,
                      transferMessageId: transferMessage.id,
                    }}
                  >
                    {transferMessage.id.slice(0, 20) + "..."}
                  </Link>
                </TableCell>
                <TableCell>
                  {transferMessage.transfer_process_id?.slice(0, 20) + "..."}
                </TableCell>
                <TableCell>{transferMessage.message_type}</TableCell>
                <TableCell>
                  {dayjs(transferMessage.created_at).format(
                    "DD/MM/YYYY - HH:mm"
                  )}
                </TableCell>
                <TableCell>{transferMessage.from}</TableCell>
                <TableCell>{transferMessage.to}</TableCell>
                <TableCell>{JSON.stringify(transferMessage.content)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table> */}
            </div>

            {/* ACTIONS */}
            <TransferProcessActions process={transferProcess} tiny={false}/>
        </div>
    );
}
