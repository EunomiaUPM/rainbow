import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table";
import {Button} from "shared/src/components/ui/button.tsx";
import {Badge, BadgeState} from "shared/src/components/ui/badge.tsx";
import {Input} from "shared/src/components/ui/input.tsx";
import {useGetContractNegotiationProcesses} from "shared/src/data/contract-queries.ts";
import {ContractNegotiationActions} from "shared/src/components/ContractNegotiationActions";
import {useMemo} from "react";
import Heading from "shared/src/components/ui/heading";
import {ArrowRight} from "lucide-react";
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
import {RouteComponent as RequestForm} from "@/routes/contract-negotiation/request";

const RouteComponent = () => {
  const {data: cnProcesses} = useGetContractNegotiationProcesses();
  const cnProcessesSorted = useMemo(() => {
    if (!cnProcesses) return [];
    return [...cnProcesses].sort((a, b) => {
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });
  }, [cnProcesses]);

  return (
    <div>
      <div className=" pb-3 flex justify-between items-start">
        <div className="basis-3/5">
          <Input type="search"></Input>
        </div>
        <Drawer direction={"right"}>
          <DrawerTrigger>
            {/* <Button>
                            Create new Request
                            <Plus className="mb-1"/>
                        </Button> */}
          </DrawerTrigger>
          <DrawerContent>
            <DrawerHeader>
              <DrawerTitle>
                <Heading level="h5" className="text-current">
                  New Contract Negotiation Request
                </Heading>
              </DrawerTitle>
            </DrawerHeader>
            <DrawerBody>
              <RequestForm/>
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
      </div>
      <Table className="text-sm">
        <TableHeader>
          <TableRow>
            <TableHead>ProviderPid</TableHead>
            <TableHead>State</TableHead>
            <TableHead>Client type</TableHead>
            <TableHead>Created at</TableHead>
            <TableHead>Actions</TableHead>
            <TableHead>Link</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {cnProcessesSorted.map((cnProcess) => (
            <TableRow key={cnProcess.provider_id?.slice(0, 20)}>
              <TableCell>
                <Badge variant={"info"}>{cnProcess.provider_id?.slice(9, 20) + "..."}</Badge>
              </TableCell>
              <TableCell>
                <Badge variant={"status"} state={cnProcess.state as BadgeState}>
                  {cnProcess.state?.replace("dspace:", "")}
                </Badge>
              </TableCell>
              <TableCell>
                <Badge>{cnProcess.is_business ? "Business" : "Standard"}</Badge>
              </TableCell>
              <TableCell>{dayjs(cnProcess.created_at).format("DD/MM/YY - HH:mm")}</TableCell>
              <TableCell>
                <ContractNegotiationActions process={cnProcess} tiny={true}/>
              </TableCell>
              <TableCell>
                <Link
                  to="/contract-negotiation/$cnProcess"
                  params={{cnProcess: cnProcess.consumer_id}}
                >
                  <Button variant="link">
                    See details
                    <ArrowRight/>
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
