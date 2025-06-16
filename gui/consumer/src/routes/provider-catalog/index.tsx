import {createFileRoute, Link} from "@tanstack/react-router";
import {ExternalLink} from "lucide-react";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {useGetParticipants} from "shared/src/data/participant-queries.ts";

const RouteComponent = () => {
    const {data: participants} = useGetParticipants();
    return <div>
        <h1>Select a provider to fetch catalogs from...</h1>
        <Table className="text-sm">
            <TableHeader>
                <TableRow>
                    <TableHead>Participant ID</TableHead>
                    <TableHead>Participant Name</TableHead>
                    <TableHead>Participant Type</TableHead>
                    <TableHead>Base URL</TableHead>
                    <TableHead>Link</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {participants.filter(p => p.participant_type == "Provider").map((participant) => (
                    <TableRow key={participant.participant_id.slice(0, 20)}>
                        <TableCell>
                            {participant.participant_id.slice(0, 20) + "..."}
                        </TableCell>
                        <TableCell>
                            {participant.participant_slug}
                        </TableCell>
                        <TableCell>
                            {participant.participant_type}
                        </TableCell>
                        <TableCell>
                            {participant.base_url}
                        </TableCell>
                        <TableCell>
                            <Link
                                to="/provider-catalog/$provider"
                                params={{provider: participant.participant_id}}
                            >
                                <ExternalLink size={12} className="text-pink-600"/>
                            </Link>
                        </TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    </div>
};

export const Route = createFileRoute("/provider-catalog/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
});
