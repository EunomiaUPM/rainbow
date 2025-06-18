import {createFileRoute} from "@tanstack/react-router";
import dayjs from "dayjs";
import {
    useGetDataplaneProcessById,
    useGetTransferMessagesByProviderPid,
    useGetTransferProcessByProviderPid,
} from "shared/src/data/transfer-queries.ts";
import {List, ListItem, ListItemKey} from "shared/src/components/ui/list.tsx";
import Heading from "shared/src/components/ui/heading.tsx";
import {Tabs, TabsContent, TabsList, TabsTrigger,} from "../../../../../shared/src/components/ui/tabs.tsx";

export const Route = createFileRoute("/transfer-process/$transferProcessId/")({
    component: RouteComponent,
});

function RouteComponent() {
    let addSpacesFormat = (text: string = "") => {
        return text.replace(/(?!^)([A-Z])/g, " $1");
    };

    const {transferProcessId} = Route.useParams();
    const {data: transferProcess} =
        useGetTransferProcessByProviderPid(transferProcessId);
    const {data: transferMessages} =
        useGetTransferMessagesByProviderPid(transferProcessId);
    const {data: dataPlane} = useGetDataplaneProcessById(transferProcessId);

    return (
        <div className="space-y-4">
            <Tabs defaultValue="data-control" className="w-[980px]">
                <TabsList>
                    <TabsTrigger value="data-control">Data Control</TabsTrigger>
                    <TabsTrigger value="data-plane">Data Plane</TabsTrigger>
                </TabsList>
                <TabsContent value="data-plane">{JSON.stringify(dataPlane)}</TabsContent>
                <TabsContent value="data-control">
                    {" "}
                    <Heading level="h5" className="mt-3">Transfer process info </Heading>
                    <div className="mb-4">
                        <List>
                            <ListItem>
                                <ListItemKey>Transfer Process Provider pid</ListItemKey>
                                <p>{transferProcess.provider_pid.slice(0, 20) + "..."}</p>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Transfer Consumer Provider pid</ListItemKey>
                                <p>{transferProcess.consumer_pid.slice(0, 20) + "..."}</p>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Agreement id</ListItemKey>
                                <p>{transferProcess.agreement_id.slice(0, 20) + "..."}</p>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Transfer Process Provider pid</ListItemKey>
                                <p>{transferProcess.state}</p>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Created At</ListItemKey>
                                <p>
                                    {" "}
                                    {dayjs(transferProcess.created_at).format("DD/MM/YY HH:mm")}
                                </p>
                            </ListItem>
                            <ListItem>
                                <ListItemKey>Updated At</ListItemKey>
                                <p>
                                    {" "}
                                    {dayjs(transferProcess.updated_at).format("DD/MM/YY HH:mm")}
                                </p>
                            </ListItem>
                        </List>
                    </div>
                    <Heading level="h5" className="text-text">
                        Transfer Messages
                    </Heading>
                    <div className="bg-white/5  rounded-md px-8 py-4 w-4/5">
                        {transferMessages.map((transferMessage) => (
                            <div className={`my-4 text-sm
               ${transferMessage.from === "Provider"
                                ? "pr-32"
                                : "pl-32"
                            }
            `}>
                                <div className={`flex w-full
              ${transferMessage.from === "Provider"
                                    ? "justify-start"
                                    : "justify-end"
                                }
                `}>
                                    <div
                                        className={`uppercase text-18 px-2 font-medium rounded-t-sm ${
                                            transferMessage.from === "Provider"
                                                ? "bg-roles-provider/20 ml-1 text-roles-provider "
                                                : "bg-roles-consumer/20 mr-1 text-roles-consumer"
                                        }`}
                                    >
                                        {transferMessage.from}
                                    </div>
                                </div>
                                <div
                                    className={` w-full px-4 py-3 rounded-md border
              ${
                                        transferMessage.from === "Provider"
                                            ? "bg-roles-provider/10 border-roles-provider/50"
                                            : "bg-roles-consumer/10 border-roles-consumer/50"
                                    }
              
              `}
                                >
                                    <div className="text-24">
                                        {" "}
                                        {addSpacesFormat(transferMessage.message_type)}{" "}

                                    </div>
                                    <p className="text-gray-100/50 mb-3">
                                        <i>
                                            {" "}
                                            {dayjs(transferMessage.created_at).format(
                                                "DD/MM/YYYY - HH:mm"
                                            )}
                                        </i>
                                    </p>
                                    <div
                                        className="flex gap-3 mb-1  text-white/60 "
                                        key={transferMessage.id.slice(0, 20)}
                                    >
                                        <p className="font-bold  min-w-40  "> Transfer Message Id </p>
                                        <p className=" w-full">
                                            {" "}
                                            {transferMessage.id.slice(9, 60)}
                                        </p>
                                    </div>
                                    <div
                                        className="flex gap-3  mb-1  text-white/60 "
                                        key={transferMessage.id.slice(9, 60)}
                                    >
                                        <p className="font-bold min-w-40"> Transfer Process Id </p>
                                        <p className=" w-full ">
                                            {" "}
                                            {transferMessage.transfer_process_id?.slice(9, 60)}
                                        </p>
                                    </div>
                                    <div
                                        className="flex flex-col gap-3 "
                                        key={transferMessage.id.slice(0, 20)}
                                    >
                                        <p className="font-bold min-w-40  text-white/60 "> Content: </p>
                                        <div className=" w-full break-all">
                                            {/* Codigo de content formateado */}
                                            <pre
                                                style={{
                                                    background: "#07070d",
                                                    padding: "1rem",
                                                    fontSize: "0.7rem",
                                                    wordBreak: "break-all",
                                                    borderRadius: "8px",
                                                }}
                                            >
                      <code
                          style={{
                              whiteSpace: "pre-wrap",
                              wordBreak: "break-word",
                              color: "#ab97ee",
                          }}
                      >
                        {JSON.stringify(transferMessage.content, null, 2)}
                      </code>
                    </pre>
                                            {/* esto por si hace falta en algn momento dividir lo que pone en el content,
                   pero por ahora no hace falta!!
                <div className="flex "> 
                  <p className="font-bold min-w-[9.4rem] "> Transfer Content: </p>
                  <p> {transferMessage.content["@context"]} </p>
                </div> 
                <div> {transferMessage.content["@type"]}  </div>  
                <div> {transferMessage.content.agreementId} </div> 
                  <div className="flex "> 
                  <p className="font-bold min-w-[9.4rem] "> Agreement ID </p>
                  <Badge variant="info"> {transferMessage.content["@context"]} </Badge>
                </div>  */}
                                        </div>
                                    </div>
                                </div>
                            </div>

                        ))}
                    </div>
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
        </div>
    );
}
