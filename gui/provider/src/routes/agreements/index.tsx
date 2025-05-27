import {createFileRoute, Link} from '@tanstack/react-router'
import {useGetAgreements} from "@/data/agreement-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";

export const Route = createFileRoute('/agreements/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {data: agreements} = useGetAgreements();
    return <div>
        <div>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Agreement Id</TableHead>
                        <TableHead>Related Message</TableHead>
                        <TableHead>Consumer Participant Id</TableHead>
                        <TableHead>Provider Participant Id</TableHead>
                        <TableHead>Status</TableHead>
                        <TableHead>CreatedAt</TableHead>
                        <TableHead>Link</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {agreements.map((agreement) => (
                        <TableRow key={agreement.agreement_id.slice(0, 20)}>
                            <TableCell>
                                {agreement.agreement_id.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {agreement.cn_message_id?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {agreement.consumer_participant_id?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {agreement.provider_participant_id?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {agreement.active ? "ACTIVE" : "INACTIVE"}
                            </TableCell>
                            <TableCell>
                                {dayjs(agreement.created_at).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                            <TableCell>
                                <Link
                                    to="/agreements/$agreementId"
                                    params={{agreementId: agreement.agreement_id}}
                                >
                                    <ExternalLink size={12} className="text-pink-600"/>
                                </Link>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    </div>
}
