import {createFileRoute} from '@tanstack/react-router'
import {useGetAgreementById} from "@/data/agreement-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";

export const Route = createFileRoute('/agreements/$agreementId')({
    component: RouteComponent,
})

function RouteComponent() {
    const {agreementId} = Route.useParams()
    const {data: agreement} = useGetAgreementById(agreementId);
    return <div className="space-y-4 pb-4">
        <div>
            Agreement with id : {agreement.agreement_id}
        </div>
        <div>
            <h2>Agreement info: </h2>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Key</TableHead>
                        <TableHead>Value</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>Agreement Id</TableCell>
                        <TableCell>{agreement.agreement_id}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Related Message</TableCell>
                        <TableCell>{agreement.cn_message_id}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Consumer Participant Id</TableCell>
                        <TableCell>{agreement.consumer_participant_id}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Provider Participant Id</TableCell>
                        <TableCell>
                            {agreement.provider_participant_id}
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Status</TableCell>
                        <TableCell>
                            {agreement.active ? "ACTIVE" : "INACTIVE"}
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Created at</TableCell>
                        <TableCell>
                            {dayjs(agreement.created_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
        <div>
            <h2>Agreement ODRL Content: </h2>
            <div>{JSON.stringify(agreement.agreement_content)}</div>
        </div>


    </div>
}
