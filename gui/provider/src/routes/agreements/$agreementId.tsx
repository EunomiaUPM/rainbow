import {createFileRoute} from '@tanstack/react-router'
import {useGetAgreementById} from "@/data/agreement-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import Heading from 'shared/src/components/ui/heading';
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";

export const Route = createFileRoute('/agreements/$agreementId')({
    component: RouteComponent,
})

function RouteComponent() {
    const {agreementId} = Route.useParams()
    const {data: agreement} = useGetAgreementById(agreementId);
    return <div className="space-y-4">
        <Heading level="h4" className="font-display">
            Agreement with id : {agreement.agreement_id}
        </Heading>
        <div>
            
            <Heading level="h5" className="text-text">Agreement info:</Heading>
            <List >
             
                    <ListItem>
                        <ListItemKey>Agreement Id</ListItemKey>
                        <TableCell>{agreement.agreement_id}</TableCell>
                    </ListItem>
                    <ListItem>
                        <ListItemKey>Related Message</ListItemKey>
                        <TableCell>{agreement.cn_message_id}</TableCell>
                    </ListItem>
                    <ListItem>
                        <ListItemKey>Consumer Participant Id</ListItemKey>
                        <TableCell>{agreement.consumer_participant_id}</TableCell>
                    </ListItem>
                    <ListItem>
                        <ListItemKey>Provider Participant Id</ListItemKey>
                        <TableCell>
                            {agreement.provider_participant_id}
                        </TableCell>
                    </ListItem>
                    <ListItem>
                        <ListItemKey>Status</ListItemKey>
                        <TableCell>
                            {agreement.active ? "ACTIVE" : "INACTIVE"}
                        </TableCell>
                    </ListItem>
                    <ListItem>
                        <ListItemKey>CreatedAt</ListItemKey>
                        <TableCell>
                            {dayjs(agreement.created_at).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </ListItem>
           
            </List>
        </div>
        <div>
           <Heading level="h5" className="text-text">Agreement content</Heading>
            <div className="max-w-[940px]">{JSON.stringify(agreement.agreement_content)}</div>
        </div>


    </div>
}
