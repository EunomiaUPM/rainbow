import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {ExternalLink} from "lucide-react";
import {Table, TableBody, p, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {getCatalogsOptions, useGetCatalogs} from "@/data/catalog-queries.ts";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";

const RouteComponent = () => {
    const {data: catalogs} = useGetCatalogs();
    return (
        <div className="space-y-4">
            <h1 className="text-xl font-bold">Catalogs</h1>
            <div>
                Main Catalog with id : {catalogs["@id"]}
            </div>
            <div>

                <h2>Main Catalog info: </h2>
                <List className="text-sm">
                     <ListItem>
                            <ListItemKey>Catalog title</ListItemKey>
                            <p>{catalogs.title}</p>
                     </ListItem>
                       <ListItem>
                            <ListItemKey>Catalog participant id</ListItemKey>
                            <p>{catalogs.participantId}</p>
                     </ListItem>
                         <ListItem>
                            <ListItemKey>Catalog homepage</ListItemKey>
                            <p>{catalogs.homepage}</p>
                        </ListItem>
                         <ListItem>
                            <ListItemKey>Catalog creation date</ListItemKey>
                            <p>
                                {dayjs(catalogs.issued).format("DD/MM/YYYY - HH:mm")}
                            </p>
                          </ListItem>
                </List>
            </div>

            <div>
                <h2>Catalogs</h2>
                <div>
                    <Heading level="h5">
                        Title
                    </Heading>
                    <p>Catalog Id </p>
                     <p>Provider ID </p>
                </div>
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
                        {/* {catalogs.catalog.map((catalog) => (
                            <TableRow key={catalog["@id"].slice(0, 20)}>
                                <p>
                                    {catalog["@id"].slice(0, 20) + "..."}
                                </p>
                                <p>
                                    {catalog.title?.slice(0, 20) + "..."}
                                </p>
                                <p>{catalog.participantId}</p>
                                <p>
                                    {dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}
                                </p>
                                <p>
                                    <Link
                                        to="/catalog/$catalogId"
                                        params={{catalogId: catalog["@id"]}}
                                    >
                                        <ExternalLink size={12} className="text-pink-600"/>
                                    </Link>
                                </p>
                            </TableRow>
                        ))} */}
                    </TableBody>
                </Table>
            </div>
        </div>
    );
};

export const Route = createFileRoute("/catalog/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
    loader: async ({context: {queryClient}}) => {
        return await queryClient.ensureQueryData(getCatalogsOptions());
    },
});
