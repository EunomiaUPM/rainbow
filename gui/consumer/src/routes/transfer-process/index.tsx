import {createFileRoute, Link} from "@tanstack/react-router";
import {useGetTransferProcesses} from "shared/src/data/transfer-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import {Button} from "shared/src/components/ui/button.tsx";
import {Badge} from "shared/src/components/ui/badge.tsx";
import {Input} from "shared/src/components/ui/input.tsx";
import {TransferProcessActions} from "shared/src/components/TransferProcessActions.tsx";
import {ArrowRight} from "lucide-react";
import {mergeStateAndAttribute} from "shared/src/lib/utils.ts";

export const Route = createFileRoute("/transfer-process/")({
    component: RouteComponent,
});

function RouteComponent() {
    const {data: transferProcesses} = useGetTransferProcesses();

    return (
        <div>
            <div className="pb-3 w-3/5">
                <Input type="search"></Input>
            </div>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Transfer Process Provider pid</TableHead>
                        <TableHead>State</TableHead>
                        <TableHead>Created at</TableHead>
                        <TableHead>Updated at</TableHead>
                        <TableHead>Actions</TableHead>
                        <TableHead>Link</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {transferProcesses.map((transferProcess) => (
                        <TableRow key={transferProcess.provider_pid.slice(0, 20)}>
                            <TableCell>
                                <Badge variant={"info"}>
                                    {transferProcess.provider_pid.slice(9, 20) + "..."}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                <Badge variant={"status"} state={transferProcess.state}>
                                    {/* TO DO STYLE: Casuística state */}
                                    {mergeStateAndAttribute(transferProcess.state, transferProcess.state_attribute)}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                {dayjs(transferProcess.created_at).format("DD/MM/YY HH:mm")}
                            </TableCell>
                            <TableCell>
                                {dayjs(transferProcess.updated_at).format("DD/MM/YY HH:mm")}
                            </TableCell>
                            <TableCell>
                                <TransferProcessActions process={transferProcess} tiny={true}/>
                            </TableCell>
                            <TableCell>

                                <Link
                                    to="/transfer-process/$transferProcessId"
                                    params={{transferProcessId: transferProcess.provider_pid}}
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
}
