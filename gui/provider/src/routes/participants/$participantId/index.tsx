import {createFileRoute, Link} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {useGetAgreementsByParticipantId, useGetParticipantById} from "@/data/participant-queries.ts";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";

export const Route = createFileRoute('/participants/$participantId/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {participantId} = Route.useParams();
    const {data: participant} = useGetParticipantById(participantId)
    const {data: agreements} = useGetAgreementsByParticipantId(participantId)
    return <div className="space-y-4">
        <div>
            Participant with id : {participant.participant_id}
        </div>
        <div>
            <h2 className="text-foreground">Transfer process info: </h2>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Key</TableHead>
                        <TableHead>Value</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>Participant ID</TableCell>
                        <TableCell>{participant.participant_id}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Identity Token</TableCell>
                        <TableCell>{participant.identity_token}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Participant Type</TableCell>
                        <TableCell>{participant._type}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Base UR</TableCell>
                        <TableCell>{participant.base_url}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Extra Info</TableCell>
                        <TableCell>
                            {JSON.stringify(participant.extra_fields)}
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>

        <div>
            <h2>Agreements</h2>
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
