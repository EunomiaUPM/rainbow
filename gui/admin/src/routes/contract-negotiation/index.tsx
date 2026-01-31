import {createFileRoute, Link} from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
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
import {ContractNegotiationActions} from "shared/src/components/actions/ContractNegotiationActions";
import {useMemo} from "react";
import {ArrowRight} from "lucide-react";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";

const RouteComponent = () => {
  const {data: cnProcesses} = useGetContractNegotiationProcesses();
  const cnProcessesSorted = useMemo(() => {
    if (!cnProcesses) return [];
    return [...cnProcesses].sort((a, b) => {
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });
  }, [cnProcesses]);

  return (
    <PageLayout>
      <PageHeader title="Contract Negotiations" />
      <PageSection>
        <div className=" pb-3 flex justify-between items-start">
          <div className=" basis-3/5">
            <Input type="search"></Input>
          </div>
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
              <TableRow key={formatUrn(cnProcess.provider_id)}>
                <TableCell>
                  <Badge variant={"info"}>{formatUrn(cnProcess.provider_id)}</Badge>
                </TableCell>
                <TableCell>
                  <Badge variant={"info"}>{formatUrn(cnProcess.consumer_id)}</Badge>
                </TableCell>
                <TableCell>
                  <Badge variant={"status"} state={cnProcess.state as BadgeState}>
                    {cnProcess.state.replace("dspace:", "")}
                  </Badge>
                </TableCell>
                <TableCell>{dayjs(cnProcess.created_at).format("DD/MM/YY - HH:mm")}</TableCell>
                <TableCell>
                  <ContractNegotiationActions process={cnProcess} tiny={true}/>
                </TableCell>
                <TableCell>
                  <Link
                    to="/contract-negotiation/$cnProcess"
                    params={{cnProcess: cnProcess.provider_id}}
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
      </PageSection>
    </PageLayout>
  );
};

export const Route = createFileRoute("/contract-negotiation/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
