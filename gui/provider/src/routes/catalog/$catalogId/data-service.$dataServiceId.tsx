import {createFileRoute} from '@tanstack/react-router'
import {getDataServiceByIdOptions, useGetDataServiceById} from "shared/src/data/catalog-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";


function RouteComponent() {
    const {dataServiceId} = Route.useParams()
    const {data: dataService} = useGetDataServiceById(dataServiceId)
    return <div className="space-y-4">
        <h2>Data service info with id: {dataService["@id"]} </h2>
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
    loader: async ({context: {queryClient}, params: {dataServiceId}}) => {
        let dataService = await queryClient.ensureQueryData(getDataServiceByIdOptions(dataServiceId))
        return {dataService};
    },
})