import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table";
import {
  getContractNegotiationMessagesByCNIDOptions,
  getContractNegotiationProcessesByCNIDOptions,
  useGetContractNegotiationMessagesByCNID,
  useGetContractNegotiationProcessesByCNID,
} from "@/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/ContractNegotiationActions.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
const RouteComponent = () => {
  let addSpacesFormat = (text) => {
    return text.replace(/(?!^)([A-Z])/g, " $1");
  };
  const { cnProcess } = Route.useParams();
  const { data } = useGetContractNegotiationProcessesByCNID(cnProcess);
  const process = data as CNProcess;
  const { data: cnMessages } =
    useGetContractNegotiationMessagesByCNID(cnProcess);

  return (
    <div className="space-y-4">
      <Heading level="h5" className="mt-3">
        Contract negotiation info{" "}
      </Heading>
      <div className="mb-4">
        <List>
          <ListItem>
            <ListItemKey>ProviderPid</ListItemKey>
            <Badge variant="info">{process.provider_id.slice(9,29) + "[...]"}</Badge>
          </ListItem>
          <ListItem>
            <ListItemKey>ConsumerPid</ListItemKey>
             <Badge variant="info">{process.consumer_id.slice(9,29) + "[...]"}</Badge>

          </ListItem>
          <ListItem>
            <ListItemKey>State</ListItemKey>
            <Badge variant="status" state={process.state.toLowerCase()}>{process.state}</Badge>
    
          </ListItem>
          <ListItem>
            <ListItemKey>CreatedAt</ListItemKey>
            <p>{dayjs(process.created_at).format("DD/MM/YYYY - HH:mm")}</p>
          </ListItem>
        </List>
      </div>
      <ContractNegotiationActions state={process.state} tiny={false} />
      <div>
        <Heading level="h5" className="text-text">
          Contract negotiation Messages
        </Heading>
        <div className="bg-white/5  rounded-md px-8 py-4 w-4/5">
          {cnMessages.map((message) => (
            <div
              className={`my-4 text-sm
               ${message.from === "Provider" ? "pr-32" : "pl-32"}
            `}
            >
              <div
                className={`flex w-full
              ${message.from === "Provider" ? "justify-start" : "justify-end"}
                `}
              >
                <div
                  className={`uppercase text-18 px-2 font-medium rounded-t-sm ${
                    message.from === "Provider"
                      ? "bg-roles-provider/20 ml-1 text-roles-provider "
                      : "bg-roles-consumer/20 mr-1 text-roles-consumer"
                  }`}
                >
                  {message.from}
                </div>
              </div>
              <div
                className={` w-full px-4 py-3 rounded-md border
              ${
                message.from === "Provider"
                  ? "bg-roles-provider/10 border-roles-provider/50"
                  : "bg-roles-consumer/10 border-roles-consumer/50"
              }
              
              `}
              >
                <div className="text-20">
                  {" "}
                  {addSpacesFormat(message._type)}{" "}
                </div>
                <p className="text-gray-100/50 mb-3">
                  <i>
                    {" "}
                    {dayjs(message.created_at).format("DD/MM/YYYY - HH:mm")}
                  </i>
                </p>
                 <div
                  className="flex gap-3 mb-1  text-white/60 "
                  key={message.cn_message_id.slice(9, 60)}
                >
                  <p className="font-bold  min-w-40  "> Contract Message Id </p>
                  <p className=" w-full">
                    {" "}
                    {message.cn_message_id.slice(9, 60) }
                  </p>
                </div>
                <div
                  className="flex gap-3 mb-1  text-white/60 "
                  key={message.cn_message_id.slice(9, 60)}
                >
                  <p className="font-bold  min-w-40  "> Contract Process Id </p>
                  <p className=" w-full">
                    {" "}
                    {message.cn_process_id.slice(9, 60) }
                  </p>
                </div>
                <div
                  className="flex flex-col gap-3 "
                 key={message.cn_message_id.slice(9, 60)}
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
                        {JSON.stringify(message.content, null, 2)}
                      </code>
                    </pre>
                    </div>
                    </div>
              </div>
            </div>
          ))}
        </div>
        {/* <h1>Messages</h1>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Message Id</TableHead>
              <TableHead>Process Id</TableHead>
              <TableHead>Type</TableHead>
              <TableHead>From</TableHead>
              <TableHead>To</TableHead>
              <TableHead>CreatedAt</TableHead>
              <TableHead>Content</TableHead>
              <TableHead>Offer</TableHead>
              <TableHead>Agreement</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {cnMessages.map((message) => (
              <TableRow key={message.cn_message_id}>
                <TableCell>
                  <Link
                    to="/contract-negotiation/$cnProcess/message/$cnMessage"
                    params={{
                      cnProcess: message.cn_process_id,
                      cnMessage: message.cn_message_id,
                    }}
                  >
                    {message.cn_message_id.slice(0, 20) + "..."}
                  </Link>
                </TableCell>
                <TableCell>
                  {message.cn_process_id.slice(0, 20) + "..."}
                </TableCell>
                <TableCell>{message._type}</TableCell>
                <TableCell>{message.from}</TableCell>
                <TableCell>{message.to}</TableCell>
                <TableCell>
                  {dayjs(message.created_at).format("DD/MM/YYYY - HH:mm")}
                </TableCell>
                <TableCell>{JSON.stringify(message.content)}</TableCell>
                <TableCell>
                  <Link to="/contract-negotiation">Offer</Link>
                </TableCell>
                <TableCell>
                  <Link to="/contract-negotiation">Agreement</Link>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table> */}
      </div>
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess/")({
  component: RouteComponent,
  loader: async ({
    context: { queryClient },
    params: { cnProcess: cnProcessId },
  }) => {
    let cnProcess = await queryClient.ensureQueryData(
      getContractNegotiationProcessesByCNIDOptions(cnProcessId as UUID)
    );
    let cnMessages = await queryClient.ensureQueryData(
      getContractNegotiationMessagesByCNIDOptions(cnProcessId as UUID)
    );
    return { cnProcess, cnMessages };
  },
});
