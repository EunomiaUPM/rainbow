import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {useGetContractNegotiationProcesses} from "shared/src/data/contract-queries.ts";
import {Button} from "shared/src/components/ui/button.tsx";
import {ContractNegotiationActions} from "shared/src/components/ContractNegotiationActions.tsx";

const RouteComponent = () => {
    const {data: cnProcesses} = useGetContractNegotiationProcesses();
    return (
        <div>
            <div className="flex justify-end">
                <Link to="/contract-negotiation/offer" className="text-decoration-none text-foreground">
                    <Button>Create new offer</Button>
                </Link>
            </div>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>ProviderPid</TableHead>
                        <TableHead>ConsumerPid</TableHead>
                        <TableHead>CreatedAt</TableHead>
                        <TableHead>Actions</TableHead>
                        <TableHead>Link</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {cnProcesses.map((cnProcess) => (
                        <TableRow key={cnProcess.provider_id.slice(0, 20)}>
                            <TableCell>
                                {cnProcess.provider_id?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {cnProcess.consumer_id?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {dayjs(cnProcess.created_at).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                            <TableCell>
                                <ContractNegotiationActions state={cnProcess.state} tiny={true}/>
                            </TableCell>
                            <TableCell>
                                <Link
                                    to="/contract-negotiation/$cnProcess"
                                    params={{cnProcess: cnProcess.consumer_id}}
                                >
                                    <ExternalLink size={12} className="text-pink-600"/>
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
