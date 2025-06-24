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
import { Button } from "shared/src/components/ui/button.tsx";
import Heading from "shared/src/components/ui/heading.tsx";
import {
  useGetContractNegotiationMessagesByCNID,
  useGetContractNegotiationProcessesByCNID,
} from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/ContractNegotiationActions.tsx";
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
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import CnProcessMessageComponent from "shared/src/components/CnProcessMessageComponent.tsx";

const RouteComponent = () => {
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
          <ListItemKey>Created at</ListItemKey>
          <ListItemDate>
            {dayjs(process.created_at).format("DD/MM/YYYY - HH:mm")}
          </ListItemDate>
        </ListItem>
      </List>
      <ContractNegotiationActions process={process} tiny={false} />
       <Drawer direction={"right"}>
          <DrawerTrigger>
            <Button variant={"secondary"}>
              See Contract Negotiation Messages
            </Button>
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
        
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess/")({
  component: RouteComponent,
});
