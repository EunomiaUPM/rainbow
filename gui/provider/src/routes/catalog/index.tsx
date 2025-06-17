import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {useGetCatalogs} from "shared/src/data/catalog-queries.ts";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import {List, ListItem, ListItemKey} from "shared/src/components/ui/list.tsx";
import {Button,} from "@/../../shared/src/components/ui/button.tsx";
import {Input,} from "@/../../shared/src/components/ui/input.tsx";

const RouteComponent = () => {
    const {data: catalogs} = useGetCatalogs();
    return (
        <div className="space-y-4">
            {/* <h1 className="text-xl font-bold">Catalogs</h1> */}
            <Heading level="h3" className="flex gap-2 items-center">
                Main Catalog with id : {catalogs["@id"]}
            </Heading>
            <div>
                <Heading level="h5">Main Catalog info: </Heading>
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
                        <p>{dayjs(catalogs.issued).format("DD/MM/YYYY - HH:mm")}</p>
                    </ListItem>
                </List>
            </div>

            <div>
                <Heading level="h5">Datasets</Heading>
                <div className='pb-3 w-3/5'>
                    <Input type="search"></Input>
                </div>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Title</TableHead>
                            <TableHead>Catalog Id</TableHead>
                            <TableHead>Provider ID</TableHead>
                            <TableHead>CreatedAt</TableHead>
                            <TableHead>Actions</TableHead>
                            <TableHead>Link</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {catalogs.catalog.map(catalogItem => (<TableRow key="urn:uuid:c4d4449d-a">

                            <TableCell>{catalogItem.title}</TableCell>
                            <TableCell>{catalogItem.title}</TableCell>
                            <TableCell>{catalogItem.participantId}</TableCell>
                            <TableCell>
                                <p className="text-18"> Dataset #1 </p>
                                <p className="text-gray-400">
                                    {" "}
                                    <i>Created at: 23/6/25 16:34 </i>
                                </p>
                            </TableCell>

                            {/*<TableCell>*/}
                            {/*    <Button>*/}
                            {/*        <Link*/}
                            {/*            to="/catalog/$catalogId"*/}
                            {/*            // params={{catalogId: catalog["@id"]}}*/}
                            {/*        >*/}
                            {/*            Create policy*/}
                            {/*        </Link>*/}
                            {/*    </Button>*/}
                            {/*</TableCell>*/}
                            <TableCell>
                                <Button>
                                    <Link
                                        to="/catalog/$catalogId"
                                        params={{catalogId: catalogItem["@id"]}}
                                    >
                                        See catalog
                                    </Link>
                                </Button>
                            </TableCell>
                        </TableRow>))}


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
});
