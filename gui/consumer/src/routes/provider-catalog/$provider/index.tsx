import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {useGetBypassCatalogs} from "../../../../../shared/src/data/catalog-bypass-queries.ts";

const RouteComponent = () => {
    const {provider} = Route.useParams()
    const {data: catalogs} = useGetBypassCatalogs(provider);
    return (
        <div className="space-y-4">
            <h1 className="text-xl font-bold">Catalogs</h1>
            <div>
                Main Catalog with id : {catalogs["@id"]}
            </div>
            <div>

                <h2>Main Catalog info: </h2>
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
                            <TableCell>{catalogs.title}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Catalog participant id</TableCell>
                            <TableCell>{catalogs.participantId}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Catalog homepage</TableCell>
                            <TableCell>{catalogs.homepage}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Catalog creation date</TableCell>
                            <TableCell>
                                {dayjs(catalogs.issued).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </div>

            <div>
                <h2>Catalogs</h2>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Catalog Id</TableHead>
                            <TableHead>Title</TableHead>
                            <TableHead>Provider ID</TableHead>
                            <TableHead>CreatedAt</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {catalogs.catalog.map((catalog) => (
                            <TableRow key={catalog["@id"].slice(0, 20)}>
                                <TableCell>
                                    {catalog["@id"].slice(0, 20) + "..."}
                                </TableCell>
                                <TableCell>
                                    {catalog.title?.slice(0, 20) + "..."}
                                </TableCell>
                                <TableCell>{catalog.participantId}</TableCell>
                                <TableCell>
                                    {dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}
                                </TableCell>
                                <TableCell>
                                    <Link
                                        to="/provider-catalog/$provider/catalog/$catalogId"
                                        params={{
                                            provider: provider,
                                            catalogId: catalog["@id"]
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
        </div>
    );
};

export const Route = createFileRoute("/provider-catalog/$provider/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
});
