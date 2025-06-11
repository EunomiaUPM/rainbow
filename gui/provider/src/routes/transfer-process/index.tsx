import {createFileRoute, Link} from '@tanstack/react-router'
import {useGetTransferProcesses} from "@/data/transfer-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";
import {Button, buttonVariants} from "shared/src/components/ui/button.tsx";
import {Input} from "shared/src/components/ui/input.tsx";

export const Route = createFileRoute('/transfer-process/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {data: transferProcesses} = useGetTransferProcesses()
    return <div>
          <div className='pb-3 w-3/5'>
            <Input type="search"></Input>
        </div>
        <Table className="text-sm">
            <TableHeader>
                <TableRow>
                    <TableHead>Transfer Process Provider pid</TableHead>
                    <TableHead>Transfer Consumer Provider pid</TableHead>
                    <TableHead>Agreement id</TableHead>
                    <TableHead>State</TableHead>
                    <TableHead>Created At</TableHead>
                    <TableHead>Updated At</TableHead>
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
                            <div className="font-badge">
                            {transferProcess.state}
                            </div>
                        </TableCell>
                        <TableCell>
                            {dayjs(transferProcess.created_at).format("DD/MM/YY HH:mm")}
                        </TableCell>
                        <TableCell>
                            {dayjs(transferProcess.updated_at).format("DD/MM/YY HH:mm")}
                        </TableCell>
                        <TableCell>
                            <Button
                                variant="default">
                            <Link
                                to="/transfer-process/$transferProcessId"
                                params={{transferProcessId: transferProcess.provider_pid}}
                            >
                                See transference
                            </Link>
                            </Button>
                        </TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    </div>
}
