import {createFileRoute, Link} from '@tanstack/react-router'
import {
    getDatasetByIdOptions,
    getDistributionsByDatasetIdOptions,
    useGetDatasetById,
    useGetDistributionsByDatasetId
} from "@/data/catalog-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";
import {getPoliciesByDatasetIdOptions, useGetPoliciesByDatasetId} from "@/data/policy-queries.ts";


function RouteComponent() {
    const {catalogId, datasetId} = Route.useParams()
    const {data: dataset} = useGetDatasetById(datasetId)
    const {data: distributions} = useGetDistributionsByDatasetId(datasetId)
    const {data: policies} = useGetPoliciesByDatasetId(datasetId)

    return <div className="space-y-4">
        <h2>Dataset info with id: {dataset["@id"]} </h2>
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
                        <TableCell>Dataset title</TableCell>
                        <TableCell>{dataset.title}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Catalog creation date</TableCell>
                        <TableCell>
                            {dayjs(dataset.issued).format("DD/MM/YYYY - HH:mm")}
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>

        <div>
            <h2>Distributions</h2>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Distribution Id</TableHead>
                        <TableHead>Distribution Title</TableHead>
                        <TableHead>CreatedAt</TableHead>
                        <TableHead>Associated Data service</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {distributions.map((distribution) => (
                        <TableRow key={distribution["@id"].slice(0, 20)}>
                            <TableCell>
                                {distribution["@id"].slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {distribution.title?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {dayjs(distribution.issued).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                            <TableCell>
                                <Link
                                    to="/catalog/$catalogId/data-service/$dataserviceId"
                                    params={{catalogId: catalogId, dataserviceId: distribution.accessService["@id"]}}
                                >
                                    <ExternalLink size={12} className="text-pink-600"/>
                                </Link>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>

        <div>
            <h2>ODRL Policies</h2>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Policy Id</TableHead>
                        <TableHead>Policy Target</TableHead>
                        <TableHead>ODRL Content</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {policies.map((policy) => (
                        <TableRow key={policy["@id"].slice(0, 20)}>
                            <TableCell>
                                {policy["@id"].slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {policy.target?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {JSON.stringify(policy)}
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    </div>

}

export const Route = createFileRoute('/catalog/$catalogId/dataset/$datasetId')({
    component: RouteComponent,
    loader: async ({context: {queryClient}, params: {datasetId}}) => {
        let dataset = await queryClient.ensureQueryData(getDatasetByIdOptions(datasetId))
        let distributions = await queryClient.ensureQueryData(getDistributionsByDatasetIdOptions(datasetId))
        let policies = await queryClient.ensureQueryData(getPoliciesByDatasetIdOptions(datasetId))
        return {dataset, distributions, policies};
    },
})