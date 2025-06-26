import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {Button} from "shared/src/components/ui/button.tsx";
import {Badge} from "shared/src/components/ui/badge.tsx";
import {Input} from "shared/src/components/ui/input.tsx";
import {useGetContractNegotiationProcesses} from "shared/src/data/contract-queries.ts";
import {ContractNegotiationActions} from "shared/src/components/ContractNegotiationActions";
import {useMemo} from "react";
import { ArrowRight, Plus } from "lucide-react";

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
            <div className="flex justify-between">
                <div className="pb-3 w-3/5">
                    <Input type="search"></Input>
                </div>
                <Link
                    to="/contract-negotiation/request"
                    className="text-decoration-none text-foreground"
                >
                    <Button>Create new request</Button>
                </Link>
            </div>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>ProviderPid</TableHead>
                        <TableHead>ConsumerPid</TableHead>
                        <TableHead>State</TableHead>
                        <TableHead>Created at</TableHead>
                        <TableHead>Actions</TableHead>
                        <TableHead>Link</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {cnProcessesSorted.map((cnProcess) => (

                        <TableRow key={cnProcess.provider_id?.slice(0, 20)}>
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
                                    {cnProcess.state}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                {dayjs(cnProcess.created_at).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                            <TableCell className="w-[270px]">
                                <ContractNegotiationActions
                                    process={cnProcess}
                                    tiny={true}
                                />
                            </TableCell>
                            <TableCell>
                                  <Link
                                        to="/contract-negotiation/$cnProcess"
                                        params={{cnProcess: cnProcess.consumer_id}}
                                    >
                                <Button variant="link">
                                  
                                        See contract 
                                       <ArrowRight />
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
