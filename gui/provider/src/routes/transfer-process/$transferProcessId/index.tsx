import {createFileRoute, Link} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import {
    useGetDataplaneProcessById,
    useGetTransferMessagesByProviderPid,
    useGetTransferProcessByProviderPid
} from "shared/src/data/transfer-queries.ts";

export const Route = createFileRoute('/transfer-process/$transferProcessId/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {transferProcessId} = Route.useParams();
    const {data: transferProcess} = useGetTransferProcessByProviderPid(transferProcessId)
    const {data: transferMessages} = useGetTransferMessagesByProviderPid(transferProcessId)
    const {data: dataplaneProcess} = useGetDataplaneProcessById(transferProcessId)

    // @ts-ignore
    // @ts-ignore
    return <div className="space-y-4">
        <div>
            Transfer process with id : {transferProcess.provider_pid}
        </div>
        <div>
            <h2>Transfer process info: </h2>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Key</TableHead>
                        <TableHead>Value</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>Transfer Process Provider pid</TableCell>
                        <TableCell>{transferProcess.provider_pid.slice(0, 20) + "..."}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Transfer Consumer Provider pid</TableCell>
                        <TableCell>{transferProcess.consumer_pid.slice(0, 20) + "..."}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Transfer Associated Consumer Id</TableCell>
                        <TableCell><Link to="/participants/$participantId" params={{
                            participantId: transferProcess.associated_consumer
                        }}>{transferProcess.associated_consumer}</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Agreement id</TableCell>
                        <TableCell>{transferProcess.agreement_id.slice(0, 20) + "..."}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>State</TableCell>
                        <TableCell>
                            {transferProcess.state}
                            {/*@ts-ignore*/}
                            {transferProcess.state == "SUSPENDED" && " - "}
                            {/*@ts-ignore*/}
                            {transferProcess.state == "SUSPENDED" && transferProcess.state_attribute}
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Created At</TableCell>
                        <TableCell>
                            {dayjs(transferProcess.created_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Updated At</TableCell>
                        <TableCell>
                            {dayjs(transferProcess.updated_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>

        <div>
            <h2>Dataplane info: </h2>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Key</TableHead>
                        <TableHead>Value</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>Process direction</TableCell>
                        <TableCell>{dataplaneProcess.process_direction}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Upstream hop</TableCell>
                        <TableCell>{dataplaneProcess.upstream_hop.url}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Downstream hop</TableCell>
                        <TableCell>{dataplaneProcess.downstream_hop.url}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Process address</TableCell>
                        <TableCell>{dataplaneProcess.process_address.url}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>State</TableCell>
                        <TableCell>{dataplaneProcess.state}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Created At</TableCell>
                        <TableCell>
                            {dayjs(dataplaneProcess.created_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Updated At</TableCell>
                        <TableCell>
                            {dayjs(dataplaneProcess.updated_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>

        <div>
            <h2>Transfer Messages</h2>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Transfer Message Id</TableHead>
                        <TableHead>Transfer Process id</TableHead>
                        <TableHead>Message type</TableHead>
                        <TableHead>Created At</TableHead>
                        <TableHead>From</TableHead>
                        <TableHead>To</TableHead>
                        <TableHead>Content</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {transferMessages.map((transferMessage) => (
                        <TableRow key={transferMessage.id.slice(0, 20)}>
                            <TableCell>
                                <Link
                                    to="/transfer-process/$transferProcessId/transfer-message/$transferMessageId"
                                    params={{
                                        transferProcessId: transferProcessId,
                                        transferMessageId: transferMessage.id
                                    }}
                                >
                                    {transferMessage.id.slice(0, 20) + "..."}
                                </Link>
                            </TableCell>
                            <TableCell>
                                {transferMessage.transfer_process_id?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>{transferMessage.message_type}</TableCell>
                            <TableCell>
                                {dayjs(transferMessage.created_at).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                            <TableCell>{transferMessage.from}</TableCell>
                            <TableCell>{transferMessage.to}</TableCell>
                            <TableCell>{JSON.stringify(transferMessage.content)}</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    </div>
}
