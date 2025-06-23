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
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "shared/src/components/ui/drawer";
import { Button } from "shared/src/components/ui/button.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { useGetContractNegotiationProcesses } from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/ContractNegotiationActions";
import { useMemo } from "react";
import { ArrowRight } from "lucide-react";
import Heading from "shared/src/components/ui/heading";
import { RouteComponent as OfferForm } from "@/routes/contract-negotiation/offer";


const RouteComponent = () => {
  const { data: cnProcesses } = useGetContractNegotiationProcesses();
  const cnProcessesSorted = useMemo(() => {
    if (!cnProcesses) return [];
    return [...cnProcesses].sort((a, b) => {
      return (
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
      );
    });
  }, [cnProcesses]);

  return (
    <div>
      <div className="flex justify-between items-start">
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <Link
          to="/contract-negotiation/offer"
          className="text-decoration-none text-foreground"
        >
          <Button>Create new offer</Button>
        </Link>
        {/* DRAWER CONTRACT OFFER*/}
        <Drawer direction={"right"}>
          <DrawerTrigger>
            <Button variant={"secondary"}>
              Create new offer
            </Button>
          </DrawerTrigger>
          <DrawerContent>
            <DrawerHeader>
              <DrawerTitle>
                <Heading level="h5" className="text-current">
                  New Contract Negotiation Offer
                </Heading>
              </DrawerTitle>
            </DrawerHeader>
            <OfferForm/>
            <DrawerFooter>
              <DrawerClose className="flex justify-start gap-4">
                <Button variant="outline" className="w-40">Cancel</Button>
                {/* <Button className="w-40">Send Offer</Button> */}
              </DrawerClose>
            </DrawerFooter>
          </DrawerContent>
        </Drawer>{/* /Drawer COotract Offer*/}
      </div>
      <Table className="text-sm">
        <TableHeader>
          <TableRow>
            <TableHead>ProviderPid</TableHead>
            <TableHead>ConsumerPid</TableHead>
            <TableHead>State</TableHead>
            <TableHead>CreatedAt</TableHead>
            <TableHead>Actions</TableHead>
            <TableHead>Link</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {cnProcessesSorted.map((cnProcess) => (
            <TableRow key={cnProcess.provider_id.slice(0, 20)}>
              <TableCell>
                <Badge variant={"info"}>
                  {cnProcess.provider_id?.slice(9, 20) + "..."}
                </Badge>
              </TableCell>
              <TableCell>
                <Badge variant={"info"}>
                  {cnProcess.consumer_id?.slice(9, 20) + "..."}
                </Badge>
              </TableCell>
              <TableCell>
                <Badge variant={"status"} state={cnProcess.state}>
                  {cnProcess.state.replace("dspace:", "")}
                </Badge>
              </TableCell>
              <TableCell>
                {dayjs(cnProcess.created_at).format("DD/MM/YY - HH:mm")}
              </TableCell>
              <TableCell>
                <ContractNegotiationActions process={cnProcess} tiny={true} />
              </TableCell>
              <TableCell>
                <Link
                  to="/contract-negotiation/$cnProcess"
                  params={{ cnProcess: cnProcess.provider_id }}
                >
                  <Button variant="link">
                    See contract negotiation
                    <ArrowRight />
                  </Button>
                </Link>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
