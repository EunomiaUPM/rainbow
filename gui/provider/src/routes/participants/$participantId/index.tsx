import {createFileRoute, Link} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {useGetAgreementsByParticipantId, useGetParticipantById} from "@/data/participant-queries.ts";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";
import  Heading  from "../../../../../shared/src/components/ui/heading.tsx";
import { Button } from 'shared/src/components/ui/button.tsx';
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
import { Badge } from 'shared/src/components/ui/badge.tsx';

export const Route = createFileRoute('/participants/$participantId/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {participantId} = Route.useParams();
    const {data: participant} = useGetParticipantById(participantId)
    const {data: agreements} = useGetAgreementsByParticipantId(participantId)
    return <div className="space-y-4">
       <Heading level="h3" >
            Participant with id : {participant.participant_id}
        </Heading>
        <div className=" flex flex-col">
            <Heading level="h6" className="text-foreground"> Participant info: </Heading>
              <div>
        
                <List>
                    <ListItem>
                        <ListItemKey>Participant ID</ListItemKey>
                        <Badge variant="info">{participant.participant_id?.slice(0,20) + "..."}</Badge>
                    </ListItem>
                    <ListItem> 
                       <ListItemKey>Identity Token</ListItemKey>
                        <p>{participant.identity_token}</p>
                    </ListItem>
                    <ListItem>
                        <ListItemKey>Participant Type</ListItemKey>
                        <p>{participant._type}</p>
                    </ListItem>
                       <ListItem>
                       <ListItemKey>Base UR</ListItemKey>
                        <p>{participant.base_url}</p>
                    </ListItem>
                       <ListItem> 
                        <ListItemKey>Extra Info</ListItemKey>
                        <p>
                            {JSON.stringify(participant.extra_fields)}
                        </p>
                    </ListItem>
                </List>
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
