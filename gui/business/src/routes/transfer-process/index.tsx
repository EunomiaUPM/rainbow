import {createFileRoute, Link} from '@tanstack/react-router'
import {useGetTransferProcesses} from "@/data/transfer-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";

export const Route = createFileRoute('/transfer-process/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {data: transferProcesses} = useGetTransferProcesses()
    return <div>
        <Table className="text-sm">
            <TableHeader>
                <TableRow>
                    <TableHead>Transfer Process Provider pid</TableHead>
                    <TableHead>Transfer Consumer Provider pid</TableHead>
                    <TableHead>Agreement id</TableHead>
                    <TableHead>State</TableHead>
                    <TableHead>Created at</TableHead>
                    <TableHead>Updated at</TableHead>
                    <TableHead>Link</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {transferProcesses.map((transferProcess) => (
                    <TableRow key={transferProcess.provider_pid.slice(0, 20)}>
                        <TableCell>
                            {transferProcess.provider_pid.slice(0, 20) + "..."}
                        </TableCell>
                        <TableCell>
                            {transferProcess.consumer_pid?.slice(0, 20) + "..."}
                        </TableCell>
                        <TableCell>
                            {transferProcess.agreement_id?.slice(0, 20) + "..."}
                        </TableCell>
                        <TableCell>
                            {transferProcess.state}
                        </TableCell>
                        <TableCell>
                            {dayjs(transferProcess.created_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                        <TableCell>
                            {dayjs(transferProcess.updated_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                        <TableCell>
                            <Link
                                to="/transfer-process/$transferProcessId"
                                params={{transferProcessId: transferProcess.provider_pid}}
                            >
                                <ExternalLink size={12} className="text-pink-600"/>
                            </Link>
                        </TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    </div>
}
