import { createFileRoute } from "@tanstack/react-router";
import dayjs from "dayjs";
import {
  useGetContractNegotiationMessagesByCNID,
  useGetContractNegotiationProcessesByCNID,
} from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/ContractNegotiationActions.tsx";
import { List, ListItem, ListItemDate, ListItemKey } from "shared/src/components/ui/list.tsx";
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
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
import CnProcessMessageComponent from "@./../../shared/src/components/CnProcessMessageComponent.tsx";

const RouteComponent = () => {
  let addSpacesFormat = (text: string) => {
    return text.replace(/(?!^)([A-Z])/g, " $1");
  };
  const { cnProcess } = Route.useParams();
  const { data } = useGetContractNegotiationProcessesByCNID(cnProcess);
  const process = data as CNProcess;
  const { data: cnMessages } = useGetContractNegotiationMessagesByCNID(cnProcess);
  useEffect(() => {
    console.log("📨 cnMessages:", cnMessages);
  }, [cnMessages]);

  return (
    <div className="space-y-4 pb-4">
      <Heading level="h5" className="mt-3">
        Contract negotiation info{" "}
      </Heading>
      <div className="mb-4 gridColsLayout">
        <List>
          <ListItem>
            <ListItemKey>ProviderPid</ListItemKey>
            <Badge variant="info">{process.provider_id.slice(9, 29) + "[...]"}</Badge>
          </ListItem>
          <ListItem>
            <ListItemKey>ConsumerPid</ListItemKey>
            <Badge variant="info">{process.consumer_id.slice(9, 29) + "[...]"}</Badge>
          </ListItem>
          <ListItem>
            <ListItemKey>State</ListItemKey>
            <Badge variant="status" state={process.state}>
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
      </div>
      <div>
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
