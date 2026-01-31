import { createFileRoute, Link } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import { useGetTransferProcesses } from "shared/src/data/transfer-queries.ts";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import { Button } from "shared/src/components/ui/button.tsx";
import { Badge, BadgeState } from "shared/src/components/ui/badge.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { TransferProcessActions } from "shared/src/components/actions/TransferProcessActions.tsx";
import { ArrowRight } from "lucide-react";
import { useMemo } from "react";
import { mergeStateAndAttribute } from "shared/src/lib/utils.ts";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";

export const Route = createFileRoute("/transfer-process/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: transferProcesses } = useGetTransferProcesses();
  const transferProcessesSorted = useMemo(() => {
    if (!transferProcesses) return [];
    return [...transferProcesses].sort((a, b) => {
      return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
    });
  }, [transferProcesses]);

  return (
    <PageLayout>
      <PageHeader title="Transfer Processes" />
      <PageSection>
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Provider pid</TableHead>
              <TableHead>State</TableHead>
              <TableHead>Created at</TableHead>
              <TableHead>Updated at</TableHead>
              <TableHead>Actions</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {transferProcessesSorted.map((transferProcess) => (
              <TableRow key={formatUrn(transferProcess.id)}>
                <TableCell>
                  <Badge variant={"info"}>{formatUrn(transferProcess.id)}</Badge>
                </TableCell>
                <TableCell>
                  <Badge variant={"status"} state={transferProcess.state as BadgeState}>
                    {mergeStateAndAttribute(transferProcess.state, transferProcess.stateAttribute)}
                  </Badge>
                </TableCell>
                <TableCell>{dayjs(transferProcess.createdAt).format("DD/MM/YY HH:mm")}</TableCell>
                <TableCell>
                  {transferProcess.updatedAt
                    ? dayjs(transferProcess.updatedAt).format("DD/MM/YY HH:mm")
                    : "-"}
                </TableCell>
                <TableCell>
                  <TransferProcessActions process={transferProcess} tiny={true} />
                </TableCell>
                <TableCell>
                  <Link
                    to="/transfer-process/$transferProcessId"
                    params={{ transferProcessId: transferProcess.id }}
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
      </PageSection>
    </PageLayout>
  );
}
