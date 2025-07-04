import {createFileRoute} from '@tanstack/react-router'
import {useGetConsumerRequests} from "shared/src/data/business-queries.ts";
import {Input} from "shared/src/components/ui/input.tsx";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {Badge} from "shared/src/components/ui/badge.tsx";
import dayjs from "dayjs";
import {useContext, useMemo} from "react";
import {AuthContext, AuthContextType} from "shared/src/context/AuthContext.tsx";
import {BusinessActions} from "../../../../shared/src/components/BusinessActions.tsx";
import {renameCNTagsForBusiness} from "@/utils";

export const Route = createFileRoute('/customer-requests/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {participant} = useContext<AuthContextType | null>(AuthContext)!
    const {data: requests} = useGetConsumerRequests(participant?.participant_id!)
    const cnProcessesSorted = useMemo(() => {
        if (!requests) return [];
        return [...requests].sort((a, b) => {
            return (
                new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
            );
        });
    }, [requests]);
    return (
        <div>
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
                        <TableHead>Created at</TableHead>
                        <TableHead>Actions</TableHead>
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
                                <Badge variant={"status"} state={cnProcess.state}>
                                    {renameCNTagsForBusiness(cnProcess.state.replace("dspace:", ""))}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                {dayjs(cnProcess.created_at).format("DD/MM/YY - HH:mm")}
                            </TableCell>
                            <TableCell>
                                <BusinessActions process={cnProcess}/>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    );
}
