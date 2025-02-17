import { getCatalogs } from "@/data/catalog-queries";
import { useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import { ExternalLink } from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table";

const RouteComponent = () => {
  const { data: cnProcesses } = useSuspenseQuery(getCatalogs);
  return (
    <div>
      <Table className="text-sm">
        <TableHeader>
          <TableRow>
            <TableHead>Contract Negotiation Process</TableHead>
            <TableHead>ProviderPid</TableHead>
            <TableHead>ConsumerPid</TableHead>
            <TableHead>State</TableHead>
            <TableHead>CreatedAt</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {cnProcesses.map((cnProcess) => (
            <TableRow key={cnProcess.cn_process_id.slice(0, 20)}>
              <TableCell>
                {cnProcess.cn_process_id.slice(0, 20) + "..."}
              </TableCell>
              <TableCell>
                {cnProcess.provider_id?.slice(0, 20) + "..."}
              </TableCell>
              <TableCell>
                {cnProcess.consumer_id?.slice(0, 20) + "..."}
              </TableCell>
              <TableCell>{cnProcess.state.replace("dspace:", "")}</TableCell>
              <TableCell>
                {dayjs(cnProcess.created_at).format("DD/MM/YYYY - HH:mm")}
              </TableCell>
              <TableCell>
                <Link
                  to="/contract-negotiation/$cnProcess"
                  params={{ cnProcess: cnProcess.cn_process_id }}
                >
                  <ExternalLink size={12} className="text-pink-600" />
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
  loader: async ({ context }) => {
    return await context.queryClient.ensureQueryData(getCatalogs);
  },
});
