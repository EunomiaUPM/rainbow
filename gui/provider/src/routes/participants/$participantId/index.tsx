import {createFileRoute, Link} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {useGetAgreementsByParticipantId, useGetParticipantById} from "@/data/participant-queries.ts";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";
import  Heading  from "../../../../../shared/src/components/ui/heading.tsx";
import { Button } from 'shared/src/components/ui/button.tsx';

export const Route = createFileRoute('/participants/$participantId/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {participantId} = Route.useParams();
    const {data: participant} = useGetParticipantById(participantId)
    const {data: agreements} = useGetAgreementsByParticipantId(participantId)
    return <div className="space-y-4">
       <Heading level="h4" >
            Participant with id : {participant.participant_id}
        </Heading>
        <div className=" flex flex-col">
            <Heading level="h6" className="text-foreground"> Participant info: </Heading>
            {/* <Table className="text-sm flex justify-center" >
                {/* <TableHeader>
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
            </Table> */}
              <div className="w-1/2 text-sm flex justify-center relative  bg-white/5 overflow-auto border border-foreground/25 rounded-md" >
        
                <ul>
                    <li className="h-6 flex flex-row justify-between gap-20 border-b border-white/20 last-child:border-0 items-center">
                        <p>Participant ID</p>
                        <p>{participant.participant_id}</p>
                    </li>
                        <li className="h-8 flex flex-row justify-between border-b border-white/20 last-child:border-0 items-center">
                       <p>Identity Token</p>
                        <p>{participant.identity_token}</p>
                    </li>
                       <li className="flex flex-row">
                        <Heading level="h6">Participant Type</Heading>
                        <p>{participant._type}</p>
                    </li>
                       <li className="flex flex-row">
                       <Heading level="h6">Base UR</Heading>
                        <p>{participant.base_url}</p>
                    </li>
                       <li className="flex flex-row">
                        <Heading level="h6">Extra Info</Heading>
                        <p>
                            {JSON.stringify(participant.extra_fields)}
                        </p>
                    </li>
                </ul>
            </div>
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
                                {dayjs(agreement.created_at).format("DD/MM/YY HH:mm")}
                            </TableCell>
                            <TableCell>
                                <Button variant="default">
                                <Link
                                    to="/agreements/$agreementId"
                                    params={{agreementId: agreement.agreement_id}}
                                >
                                      See agreement
                                </Link>
                              
                                 </Button>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    </div>
}
