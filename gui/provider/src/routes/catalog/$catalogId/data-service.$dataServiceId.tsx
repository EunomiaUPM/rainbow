import {createFileRoute} from '@tanstack/react-router'
import {useGetDataServiceById} from "shared/src/data/catalog-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import Heading from 'shared/src/components/ui/heading';
import { Badge } from "shared/src/components/ui/badge";

function RouteComponent() {
    const {dataServiceId} = Route.useParams()
    const {data: dataService} = useGetDataServiceById(dataServiceId)
    return <div className="space-y-4">
        <Heading level="h3" className="flex gap-2 items-center">
        Data service info with id
        <Badge variant="info" size="lg"> {dataService["@id"].slice(9,29)+ "[...]"}
            </Badge> </Heading>
        <div>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Key</TableHead>
                        <TableHead>Value</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>Data service title</TableCell>
                        <TableCell>{dataService.title}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Data service creation date</TableCell>
                        <TableCell>
                            {dayjs(dataService.issued).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Data service endpoint URL</TableCell>
                        <TableCell>
                            <TableCell>{dataService.endpointURL}</TableCell>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Data service description</TableCell>
                        <TableCell>
                            <TableCell>{dataService.endpointDescription}</TableCell>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
    </div>
}

export const Route = createFileRoute('/catalog/$catalogId/data-service/$dataServiceId')({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,

})