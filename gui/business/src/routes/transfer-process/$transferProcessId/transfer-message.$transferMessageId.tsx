import {createFileRoute} from '@tanstack/react-router'
import {useGetTransferMessageById} from "@/data/transfer-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";

export const Route = createFileRoute(
    '/transfer-process/$transferProcessId/transfer-message/$transferMessageId',
)({
    component: RouteComponent,
})

function RouteComponent() {
    const {transferProcessId, transferMessageId} = Route.useParams();
    const {data: transferMessage} = useGetTransferMessageById(transferProcessId, transferMessageId)
    return (
        <div className="space-y-4">
            <div>
                Transfer process message with id : {transferMessage.id}
            </div>
            <div>
                <h2>Transfer message info: </h2>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Key</TableHead>
                            <TableHead>Value</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>Transfer Message Id</TableCell>
                            <TableCell>{transferMessage.id}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Transfer Process id</TableCell>
                            <TableCell>{transferMessage.transfer_process_id}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Message type</TableCell>
                            <TableCell>{transferMessage.message_type}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Created At</TableCell>
                            <TableCell>{dayjs(transferMessage.created_at).format("DD/MM/YYYY - HH:mm")}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>From</TableCell>
                            <TableCell>{transferMessage.from}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>To</TableCell>
                            <TableCell>{transferMessage.to}</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </div>
            <pre className="whitespace-pre-wrap">{JSON.stringify(transferMessage.content)}</pre>
        </div>
    )
}
