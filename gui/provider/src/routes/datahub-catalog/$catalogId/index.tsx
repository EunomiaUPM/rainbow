import {createFileRoute, Link} from "@tanstack/react-router";
import {useGetDatahubDatasetsByCatalogId} from "shared/src/data/datahub-catalog-queries";
import Heading from "shared/src/components/ui/heading.tsx";
import {Input} from "shared/src/components/ui/input.tsx";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {Button} from "shared/src/components/ui/button.tsx";
import {
    Drawer,
    DrawerBody,
    DrawerClose,
    DrawerContent,
    DrawerFooter,
    DrawerHeader,
    DrawerTitle,
    DrawerTrigger
} from "shared/src/components/ui/drawer.tsx";
import {useGetParticipants} from "shared/src/data/participant-queries.ts";
import {OfferDrawer} from "@/components/OfferDrawer.tsx";

const RouteComponent = () => {
    const {catalogId} = Route.useParams();
    const {data: datasets} = useGetDatahubDatasetsByCatalogId(catalogId);

    const {data: participants} = useGetParticipants();


    return (
        <div className="space-y-4">
            <div>
                <Heading level="h5">Datasets</Heading>
                <div className='pb-3 w-3/5'>
                    <Input type="search"></Input>
                </div>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Id</TableHead>
                            <TableHead>Name</TableHead>
                            <TableHead>ETL system</TableHead>
                            <TableHead>Description</TableHead>
                            <TableHead>Tags</TableHead>
                            <TableHead>Glossary</TableHead>
                            <TableHead>Offer</TableHead>
                            <TableHead>Link</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {datasets.map(dataset => (<TableRow key="urn:uuid:c4d4449d-a">

                            <TableCell>{dataset.urn.slice(0, 15)}...</TableCell>
                            <TableCell>{dataset.name}</TableCell>
                            <TableCell>{dataset.platform.name}</TableCell>
                            <TableCell>{dataset.description}</TableCell>
                            <TableCell>{dataset.tag_names.join(", ")}</TableCell>
                            <TableCell>{dataset.glossary_terms.map(m => (
                                <span>{m.glossaryTermInfo.name}</span>
                            ))}</TableCell>
                            <TableCell>
                                <Drawer direction={"right"}>
                                    <DrawerTrigger>
                                        <Button variant="outline" size="sm">
                                            + Offer dataset
                                        </Button>
                                    </DrawerTrigger>
                                    <DrawerContent>
                                        <DrawerHeader>
                                            <DrawerTitle>
                                                <Heading level="h5" className="text-current">
                                                    New Contract Negotiation Offer
                                                </Heading>
                                            </DrawerTitle>
                                        </DrawerHeader>
                                        <DrawerBody>
                                            <OfferDrawer catalogId={catalogId} datasetId={dataset.urn}/>
                                        </DrawerBody>
                                        <DrawerFooter>
                                            <DrawerClose className="flex justify-start gap-4">
                                                <Button variant="ghost" className="w-40">
                                                    Cancel
                                                </Button>
                                            </DrawerClose>
                                        </DrawerFooter>
                                    </DrawerContent>
                                </Drawer>
                            </TableCell>
                            <TableCell>
                                <Button>
                                    <Link
                                        to="/datahub-catalog/$catalogId/dataset/$datasetId"
                                        params={{catalogId: catalogId, datasetId: dataset.urn}}
                                    >
                                        See dataset
                                    </Link>
                                </Button>
                            </TableCell>
                        </TableRow>))}
                    </TableBody>
                </Table>
            </div>
        </div>
    );
};

export const Route = createFileRoute("/datahub-catalog/$catalogId/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
});
