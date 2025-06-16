import { createFileRoute, Link } from "@tanstack/react-router";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import {
  useGetTransferMessagesByProviderPid,
  useGetTransferProcessByProviderPid,
} from "@/data/transfer-queries.ts";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "../../../../../shared/src/components/ui/tabs.tsx";

export const Route = createFileRoute("/transfer-process/$transferProcessId/")({
  component: RouteComponent,
});

function RouteComponent() {
  let addSpacesFromat = (text) => {
    return text.replace(/(?!^)([A-Z])/g, " $1");
  };
  const { transferProcessId } = Route.useParams();
  const { data: transferProcess } =
    useGetTransferProcessByProviderPid(transferProcessId);
  const { data: transferMessages } =
    useGetTransferMessagesByProviderPid(transferProcessId);
  return (
    <div className="space-y-4">
      <Tabs defaultValue="account" className="w-[400px]">
        <TabsList>
          <TabsTrigger value="data-control">Data Control</TabsTrigger>
          <TabsTrigger value="data-plane">Data Plane</TabsTrigger>
        </TabsList>
        <TabsContent value="data-plane">// Data plane</TabsContent>
        <TabsContent value="data-control">
          {" "}
          <Heading level="h6">Transfer process info: </Heading>
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
        </TabsContent>
      </Tabs>
      <div>
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
        <Heading level="h6" className="text-text">
          Transfer Messages
        </Heading>
        {transferMessages.map((transferMessage) => (
          <div className="my-4">
            <div
              className={`uppercase inline-block  ${
                transferMessage.from === "Provider"
                  ? "bg-provider-800 "
                  : "bg-consumer-800"
              }`}
            >
              {transferMessage.from}
            </div>
            <div className={` w-3/5  p-2 rounded-md 
              ${ transferMessage.from === "Provider"
                ? "bg-provider-800" 
                 : "bg-consumer-800"
              }
              
              ` }>
              <div className="text-20">
                {" "}
                {addSpacesFromat(transferMessage.message_type)}{" "}
              </div>
              <p>
                <i>
                  {" "}
                  {dayjs(transferMessage.created_at).format(
                    "DD/MM/YYYY - HH:mm"
                  )}
                </i>
              </p>
              <div
                className="flex gap-3 "
                key={transferMessage.id.slice(0, 20)}
              >
                <p className="font-bold "> Transfer Message Id </p>
                <p className=" w-1/2">
                  {" "}
                  {transferMessage.id.slice(0, 20) + "..."}
                </p>
              </div>
              <div
                className="flex gap-3 "
                key={transferMessage.id.slice(0, 20)}
              >
                <p className="font-bold "> Transfer Process Id </p>
                <p className=" w-1/2">
                  {" "}
                  {transferMessage.transfer_process_id?.slice(0, 20) + "..."}
                </p>
              </div>
              <div
                className="flex gap-3 "
                key={transferMessage.id.slice(0, 20)}
              >
                <p className="font-bold "> Transfer Process Id </p>
                <p className=" w-1/2">
                {JSON.stringify(transferMessage.content)}
                </p>
              </div>
            </div>
          </div>
        ))}
        <Table className="text-sm">
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
        </Table>
      </div>
    </div>
  );
}
