import { createFileRoute } from "@tanstack/react-router";
import dayjs from "dayjs";
import {
  useGetContractNegotiationMessagesByCNID,
  useGetContractNegotiationProcessesByCNID,
} from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/ContractNegotiationActions.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "@./../../shared/src/components/ui/drawer.tsx";
import { CnProcessMessageContainer } from "@./../../shared/src/components/CnProcessMessageContainer.tsx"
import { Button } from "shared/src/components/ui/button.tsx";

const RouteComponent = () => {
  let addSpacesFormat = (text: string) => {
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
            <Badge variant="info">
              {process.provider_id.slice(9, 29) + "[...]"}
            </Badge>
          </ListItem>
          <ListItem>
            <ListItemKey>ConsumerPid</ListItemKey>
            <Badge variant="info">
              {process.consumer_id.slice(9, 29) + "[...]"}
            </Badge>
          </ListItem>
          <ListItem>
            <ListItemKey>State</ListItemKey>
            <Badge variant="status" state={process.state}>
              {process.state}
            </Badge>
          </ListItem>
          <ListItem>
            <ListItemKey>CreatedAt</ListItemKey>
            <p>{dayjs(process.created_at).format("DD/MM/YYYY - HH:mm")}</p>
          </ListItem>
        </List>
      </div>
      <div>
        {/*  */}

        {/* DRAWER */}
        <Drawer direction={"right"}>
          <DrawerTrigger>
            <Button variant={"secondary"}>
              See Contract Negociation Messages
            </Button>
          </DrawerTrigger>
          <DrawerContent>
            <DrawerHeader>
              <DrawerTitle>
                <Heading level="h5"  className="text-current">
                  Contract negotiation Messages
                </Heading>
              </DrawerTitle>
            </DrawerHeader>
            <div className="bg-white/5  rounded-md px-6 py-4">
              {cnMessages.map((message) => (
                <CnProcessMessageContainer key={message.cn_message_id} message={message} />
            //     <div
            //       className={`my-4 text-sm
            //    ${message.from === "Provider" ? "pr-12" : "pl-12"}
            // `}
            //     >
            //       <div
            //         className={`flex w-full
            //   ${message.from === "Provider" ? "justify-start" : "justify-end"}
            //     `}
            //       >
            //         <div
            //           className={`uppercase text-18 px-2 font-medium rounded-t-sm ${
            //             message.from === "Provider"
            //               ? "bg-roles-provider/20 ml-1 text-roles-provider "
            //               : "bg-roles-consumer/20 mr-1 text-roles-consumer"
            //           }`}
            //         >
            //           {message.from}
            //         </div>
            //       </div>
            //       <div
            //         className={` w-full px-4 py-3 rounded-md rounded-b-xl border
            //   ${
            //     message.from === "Provider"
            //       ? "bg-roles-provider/10 border-roles-provider/50"
            //       : "bg-roles-consumer/10 border-roles-consumer/50"
            //   }
            //   `}
            //       >
            //         <div className="text-20">
            //           {addSpacesFormat(message._type)}{" "}
            //         </div>
            //         <p className="text-gray-100/50 mb-3">
            //           <i>
            //             {dayjs(message.created_at).format("DD/MM/YYYY - HH:mm")}
            //           </i>
            //         </p>
            //         <div
            //           className="flex gap-3 mb-1  text-white/60 "
            //           key={message.cn_message_id.slice(9, 60)}
            //         >
            //           <p className="font-bold  min-w-40  ">
            //             Contract Message Id{" "}
            //           </p>
            //           <p className=" w-full">
            //             {message.cn_message_id.slice(9, 60)}
            //           </p>
            //         </div>
            //         <div
            //           className="flex gap-3 mb-1  text-white/60 "
            //           key={message.cn_message_id.slice(9, 60)}
            //         >
            //           <p className="font-bold  min-w-40  ">
            //             Contract Process Id{" "}
            //           </p>
            //           <p className=" w-full">
            //             {message.cn_process_id.slice(9, 60)}
            //           </p>
            //         </div>
            //         <div
            //           className="flex flex-col gap-3 "
            //           key={message.cn_message_id.slice(9, 60)}
            //         >
            //           <p className="font-bold min-w-40  text-white/60 ">
            //             Content:{" "}
            //           </p>
            //           <div className=" w-full break-all">
            //             {/* Codigo de content formateado */}
            //             <pre className="p-4 rounded-lg break-all text-[11px] bg-black/70 text-secondary-400 break-all">
            //               <code className="whitespace-pre-wrap break-all">
            //                 {JSON.stringify(message.content, null, 2)}
            //               </code>
            //             </pre>
            //           </div>
            //         </div>
            //       </div>
            //     </div>
              ))}
            </div>
            <DrawerFooter>
              <DrawerClose>
                <Button variant="outline">Hide Messages</Button>
              </DrawerClose>
            </DrawerFooter>
          </DrawerContent>
        </Drawer>
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

      {/* ACTIONS */}
      <ContractNegotiationActions process={process} tiny={false} />
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess/")({
  component: RouteComponent,
});
