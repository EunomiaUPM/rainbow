import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import { ExternalLink } from "lucide-react";

// Components
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table";
import {
  getContractNegotiationProcessesOptions,
  useGetContractNegotiationProcesses,
} from "@/data/contract-queries.ts";
import { Button, buttonVariants } from "shared/src/components/ui/button.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { ContractNegotiationActions } from "shared/src/components/ContractNegotiationActions.tsx";
import { Input } from "shared/src/components/ui/input.tsx";

const RouteComponent = () => {
  const { data: cnProcesses } = useGetContractNegotiationProcesses();


  return (
    <div>
      <div className="flex justify-between">
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <Link
          to="/contract-negotiation/offer"
          className="text-decoration-none text-foreground"
        >
          <Button>Create new offer</Button>
        </Link>
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
          {cnProcesses.map((cnProcess) => (
           
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
                <Badge variant={"status"} state={'success'}>
                  {cnProcess.state.replace("dspace:", "")}
                </Badge>
              </TableCell>
              <TableCell>
                {dayjs(cnProcess.created_at).format("DD/MM/YYYY - HH:mm")}
              </TableCell>
              <TableCell>
                <ContractNegotiationActions
                  state={cnProcess.state}
                  tiny={true}
                />
              </TableCell>
              <TableCell>
                <Button variant="default">
                  <Link
                    to="/contract-negotiation/$cnProcess"
                    params={{ cnProcess: cnProcess.provider_id }}
                  >
                    See contract negotiation
                  </Link>
                </Button>
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
  loader: async ({ context: { queryClient } }) => {
    let cnProcesses = await queryClient.ensureQueryData(
      getContractNegotiationProcessesOptions()
    );
    return { cnProcesses };
  },
});
