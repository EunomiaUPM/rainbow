import { createFileRoute, Link } from "@tanstack/react-router";
import { useState } from "react";
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
  DrawerBody,
  DrawerTitle,
  DrawerTrigger,
} from "shared/src/components/ui/drawer";
import { Button } from "shared/src/components/ui/button.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { useGetContractNegotiationProcesses } from "shared/src/data/contract-queries.ts";
import { ContractNegotiationActions } from "shared/src/components/ContractNegotiationActions";
import { useMemo } from "react";
import { ArrowRight, Plus } from "lucide-react";
import Heading from "shared/src/components/ui/heading";
import { RouteComponent as OfferForm } from "@/routes/contract-negotiation/offer";
import { RouteComponent as CounterOfferForm } from "@/routes/contract-negotiation/counter-offer";

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
  const [openCounterOffer, setOpenCounterOffer] = useState(false);
  const [selectedCnProcess, setSelectedCnProcess] = useState("");
  const mockeoProcess = {
    associated_consumer:
      "did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5Iiwia2lkIjoiY1BSUFJYVXFjM0htWVdNeEZHZWFVVWlpTjVONkZaM2dQZHg1V2JnaG96NCIsIngiOiJHMGFyc1RUSzUzUnhLYldDemlLcGpuYjQ4NWdLdkh1REhZcFJ5RXJmRnIwIn0",
    consumer_id: "urn:uuid:d0518bd5-6e77-4a4d-98d9-0290cf9eda67",
    created_at: "2025-07-04T09:22:01.898706",
    initiated_by: "Provider",
    is_business: false,
    provider_id: "urn:uuid:f8e03608-b34a-44bd-991b-9443fb439323",
    state: "REQUESTED",
    updated_at: null,
  };

  return (
    <div>
      <div
        className={
          openCounterOffer === true ? `flex fixed   z-50` : `hidden absolute`
        }
      >
        <CounterOfferForm process={selectedCnProcess} />
      </div>
      <div className=" pb-3 flex justify-between items-start">
        <div className=" basis-3/5">
          <Input type="search"></Input>
        </div>

        {/* DRAWER CONTRACT OFFER*/}
        <Drawer direction={"right"}>
          {/* <DrawerTrigger>
            <Button>
              Create new offer
              <Plus className="mb-1" />
            </Button>
          </DrawerTrigger> */}
          <DrawerContent>
            <DrawerHeader>
              <DrawerTitle>
                <Heading level="h5" className="text-current">
                  New Contract Negotiation Offer
                </Heading>
              </DrawerTitle>
            </DrawerHeader>
            <DrawerBody>
              <OfferForm />
            </DrawerBody>
            <DrawerFooter>
              <DrawerClose className="flex justify-start gap-4">
                <Button variant="ghost" className="w-40">
                  Cancel
                </Button>
              </DrawerClose>
            </DrawerFooter>
          </DrawerContent>
        </Drawer>
        {/* /Drawer Contract Offer*/}
      </div>
      <Table className="text-sm">
        <TableHeader>
          <TableRow>
            <TableHead>ProviderPid</TableHead>
            {/* <TableHead>ConsumerPid</TableHead> */}
            <TableHead>State</TableHead>
            <TableHead>Client Type</TableHead>
            <TableHead>Created at</TableHead>
            <TableHead>Actions</TableHead>
            <TableHead>Link</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {cnProcessesSorted.map((cnProcess) => (
            <TableRow key={cnProcess.provider_id?.slice(0, 20)}>
              {console.log(cnProcess, "cnProcess console")}
              <TableCell>
                <Badge variant={"info"}>
                  {cnProcess.provider_id?.slice(9, 20) + "..."}
                </Badge>
              </TableCell>
              {/* <TableCell>
                <Badge variant={"info"}>
                  {cnProcess.consumer_id?.slice(9, 20) + "..."}
                </Badge>
              </TableCell> */}
              <TableCell>
                <Badge variant={"status"} state={cnProcess.state}>
                  {cnProcess.state.replace("dspace:", "")}
                </Badge>
              </TableCell>
              <TableCell>
                <Badge>{cnProcess.is_business ? "Business" : "Standard"}</Badge>
              </TableCell>
              <TableCell>
                {dayjs(cnProcess.created_at).format("DD/MM/YY - HH:mm")}
              </TableCell>
              <TableCell>
                {/* {console.log(selectedCnProcess, " selected cnproc")}
                                {cnProcess.state === "REQUESTED" ?
                                    <Button variant="outline" size="sm" className="" onClick={() =>
                                        // toggle dialog
                                    {
                                        (openCounterOffer === true ? setOpenCounterOffer(false) : setOpenCounterOffer(true));
                                        (setSelectedCnProcess(cnProcess))
                                    }


                                    }>
                                        C/Offer
                                    </Button> : ""} */}

                <ContractNegotiationActions process={cnProcess} tiny={true} />
              </TableCell>
              <TableCell>
                <Link
                  to="/contract-negotiation/$cnProcess"
                  params={{ cnProcess: cnProcess.provider_id }}
                >
                  <Button variant="link">
                    See details
                    <ArrowRight />
                  </Button>
                </Link>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
       
      </Table>
{/*     esto es para pruebas, cuando consumer pueda acceder a catálogo 
de provider, se borra
       <div>
          <ContractNegotiationActions process={mockeoProcess} tiny={true}/>
        </div> */}
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
