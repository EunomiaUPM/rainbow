import {createFileRoute, Link} from '@tanstack/react-router'
import {useGetParticipants} from "@/data/participant-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {ExternalLink} from "lucide-react";
import {useContext} from "react";
import {PubSubContext} from "@/context/PubSubContext.tsx";

export const Route = createFileRoute('/participants/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {data: participants} = useGetParticipants()
    const {lastHighLightedNotification} = useContext(PubSubContext)!;
    return <div>
        <Table className="text-sm">
            <TableHeader>
                <TableRow>
                    <TableHead>Participant ID</TableHead>
                    <TableHead>Identity Token</TableHead>
                    <TableHead>Participant Type</TableHead>
                    <TableHead>Base URL</TableHead>
                    <TableHead>Extra Info</TableHead>
                    <TableHead>Link</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {participants.map((participant) => (
                    <TableRow key={participant.participant_id.slice(0, 20)} className={
                        participant.participant_id === lastHighLightedNotification
                            ? "bg-blue-200"
                            : ""
                    }>
                        <TableCell>
                            {participant.participant_id.slice(0, 20) + "..."}
                        </TableCell>
                        <TableCell>
                            {participant.identity_token?.slice(0, 20) + "..."}
                        </TableCell>
                        <TableCell>
                            {participant._type}
                        </TableCell>
                        <TableCell>
                            {participant.base_url}
                        </TableCell>
                        <TableCell>
                            {JSON.stringify(participant.extra_fields)}
                        </TableCell>
                        <TableCell>
                            <Link
                                to="/participants/$participantId"
                                params={{participantId: participant.participant_id}}
                            >
                                <ExternalLink size={12} className="text-pink-600"/>
                            </Link>
                        </TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    </div>
}
