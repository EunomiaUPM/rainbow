import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {
    useGetContractNegotiationMessagesByCNID,
    useGetContractNegotiationProcessesByCNID
} from "shared/src/data/contract-queries.ts";
import {ContractNegotiationActions} from "shared/src/components/ContractNegotiationActions.tsx";

const RouteComponent = () => {
    const {cnProcess} = Route.useParams();
    const {data: process} = useGetContractNegotiationProcessesByCNID(cnProcess);
    const {data: cnMessages} = useGetContractNegotiationMessagesByCNID(cnProcess);

    return (
        <div className="space-y-4">
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Key</TableHead>
                        <TableHead>Value</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>ProviderPid</TableCell>
                        <TableCell>{process.provider_id}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>ConsumerPid</TableCell>
                        <TableCell>{process.consumer_id}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>State</TableCell>
                        <TableCell>{process.state}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>CreatedAt</TableCell>
                        <TableCell>
                            {dayjs(process.created_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
            <ContractNegotiationActions state={process.state} tiny={false}/>
            <div>
                <h1>Messages</h1>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Message Id</TableHead>
                            <TableHead>Process Id</TableHead>
                            <TableHead>Type</TableHead>
                            <TableHead>From</TableHead>
                            <TableHead>To</TableHead>
                            <TableHead>CreatedAt</TableHead>
                            <TableHead>Content</TableHead>
                            <TableHead>Offer</TableHead>
                            <TableHead>Agreement</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {cnMessages.map((message) => (
                            <TableRow key={message.cn_message_id}>
                                <TableCell>
                                    <Link
                                        to="/contract-negotiation/$cnProcess/message/$cnMessage"
                                        params={{
                                            cnProcess: message.cn_process_id,
                                            cnMessage: message.cn_message_id
                                        }}
                                    >
                                        {message.cn_message_id.slice(0, 20) + "..."}
                                    </Link>
                                </TableCell>
                                <TableCell>{message.cn_process_id.slice(0, 20) + "..."}</TableCell>
                                <TableCell>{message._type}</TableCell>
                                <TableCell>{message.from}</TableCell>
                                <TableCell>{message.to}</TableCell>
                                <TableCell>
                                    {dayjs(message.created_at).format("DD/MM/YYYY - HH:mm")}
                                </TableCell>
                                <TableCell>{JSON.stringify(message.content)}</TableCell>
                                <TableCell><Link to="/contract-negotiation">Offer</Link></TableCell>
                                <TableCell><Link to="/contract-negotiation">Agreement</Link></TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>
        </div>
    );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess/")({
    component: RouteComponent,
});
