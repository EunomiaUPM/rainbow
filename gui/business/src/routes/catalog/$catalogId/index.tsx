import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {
    getCatalogsByIdOptions,
    getDataServicesByCatalogIdOptions,
    getDatasetsByCatalogIdOptions,
    useGetCatalogsById,
    useGetDataServicesByCatalogId,
    useGetDatasetsByCatalogId
} from "@/data/catalog-queries.ts";

const RouteComponent = () => {
    const {catalogId} = Route.useParams();
    const {data: catalog} = useGetCatalogsById(catalogId);
    const {data: datasets} = useGetDatasetsByCatalogId(catalogId);
    const {data: dataservices} = useGetDataServicesByCatalogId(catalogId);

    return (
        <div className="space-y-4 pb-4">
            <h1 className="text-xl font-bold">Catalogs</h1>
            <div>
                Catalog with id : {catalog["@id"]}
            </div>
            <div>

                <h2>Catalog info: </h2>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Key</TableHead>
                            <TableHead>Value</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>Catalog title</TableCell>
                            <TableCell>{catalog.title}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Catalog participant id</TableCell>
                            <TableCell>{catalog.participantId}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Catalog homepage</TableCell>
                            <TableCell>{catalog.homepage}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Catalog creation date</TableCell>
                            <TableCell>
                                {dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </div>

            <div>
                <h2>Datasets</h2>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Dataset ID</TableHead>
                            <TableHead>Title</TableHead>
                            <TableHead>Provider ID</TableHead>
                            <TableHead>Created at</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {datasets.map((dataset) => (
                            <TableRow key={dataset["@id"].slice(0, 20)}>
                                <TableCell>
                                    {dataset["@id"].slice(0, 20) + "..."}
                                </TableCell>
                                <TableCell>
                                    {dataset.title?.slice(0, 20) + "..."}
                                </TableCell>
                                <TableCell>{catalog.participantId}</TableCell>
                                <TableCell>
                                    {dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}
                                </TableCell>
                                <TableCell>
                                    <Link
                                        to="/catalog/$catalogId/dataset/$datasetId"
                                        params={{
                                            catalogId: catalog["@id"],
                                            datasetId: dataset["@id"]
                                        }}
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
                <h2>Dataservices</h2>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Dataservice ID</TableHead>
                            <TableHead>Endpoint</TableHead>
                            <TableHead>Endpoint Description</TableHead>
                            <TableHead>Created at</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {dataservices.map((dataservice) => (
                            <TableRow key={dataservice["@id"].slice(0, 20)}>
                                <TableCell>
                                    {dataservice["@id"].slice(0, 20) + "..."}
                                </TableCell>
                                <TableCell>{dataservice.endpointURL}</TableCell>
                                <TableCell>{dataservice.endpointDescription}</TableCell>
                                <TableCell>
                                    {dayjs(dataservice.issued).format("DD/MM/YYYY - HH:mm")}
                                </TableCell>
                                <TableCell>
                                    <Link
                                        to="/catalog/$catalogId/data-service/$dataserviceId"
                                        params={{catalogId: catalog["@id"], dataserviceId: dataservice["@id"]}}
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
    );
};

export const Route = createFileRoute("/catalog/$catalogId/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
    loader: async ({context: {queryClient}, params: {catalogId}}) => {
        let catalog = await queryClient.ensureQueryData(getCatalogsByIdOptions(catalogId))
        let datasets = await queryClient.ensureQueryData(getDatasetsByCatalogIdOptions(catalogId))
        let dataServices = await queryClient.ensureQueryData(getDataServicesByCatalogIdOptions(catalogId))
        return {catalog, datasets, dataServices};
    },
});
