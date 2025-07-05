import {createFileRoute, Link} from "@tanstack/react-router";
import {useState} from "react";
import Heading from "shared/src/components/ui/heading.tsx";
import {Input} from "shared/src/components/ui/input.tsx";
import {Badge} from "shared/src/components/ui/badge";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table.tsx";
import {Button} from "shared/src/components/ui/button.tsx";
import {
    Drawer,
    DrawerBody,
    DrawerContent,
    DrawerHeader,
    DrawerTitle,
    DrawerTrigger,
} from "shared/src/components/ui/drawer";
import {useGetParticipants} from "shared/src/data/participant-queries.ts";
import {ArrowRight} from "lucide-react";
import {useGetDatahubBypassDatasetsByCatalogId} from "shared/src/data/catalog-datahub-bypass-queries.ts";

const RouteComponent = () => {
    const [open, setOpen] = useState(false)
    const {provider, catalogId} = Route.useParams();
    const {data: datasets} = useGetDatahubBypassDatasetsByCatalogId(provider, catalogId);
    const {data: participants} = useGetParticipants();

    return (
        <div className="space-y-4 pb-4">
            <div>
                <Heading level="h5">Datasets</Heading>
                <div className="pb-3 w-3/5">
                    <Input type="search"></Input>
                </div>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            {/* <TableHead>Id</TableHead> */}
                            <TableHead>Name</TableHead>
                            <TableHead>ETL system</TableHead>
                            <TableHead>Description</TableHead>
                            {/* <TableHead>Tags</TableHead> */}
                            <TableHead>Glossary</TableHead>
                            <TableHead>Request</TableHead>
                            <TableHead>Link</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {datasets.map((dataset) => (
                            <TableRow key="urn:uuid:c4d4449d-a">
                                {/* <TableCell>{dataset.urn.slice(19,25)}...</TableCell> */}
                                <TableCell>{dataset.name}</TableCell>
                                <TableCell>
                                    <Badge variant="info">
                                        {dataset.platform.name}
                                    </Badge>
                                </TableCell>
                                <TableCell className="min-w-[220px]">
                                    <p className="text-xs"> {dataset.description}</p>
                                </TableCell>
                                {/* <TableCell>{dataset.tag_names.join(", ")}</TableCell> */}
                                <TableCell>
                                    <Badge>
                                        {dataset.glossary_terms.map((m) => (
                                            <span>{m.glossaryTermInfo.name}</span>
                                        ))}
                                    </Badge>
                                </TableCell>
                                <TableCell>
                                    <Drawer direction={"right"}>
                                        <DrawerTrigger>
                                            <Button variant="outline" size="sm">
                                                + Request dataset
                                            </Button>
                                        </DrawerTrigger>
                                        <DrawerContent>
                                            <DrawerHeader>
                                                <DrawerTitle>
                                                    <Heading level="h5" className="text-current">
                                                        New Contract Negotiation Request
                                                    </Heading>
                                                </DrawerTitle>
                                            </DrawerHeader>
                                            <DrawerBody className="items-start">
                                                ... request drawer
                                            </DrawerBody>
                                            {/* <DrawerFooter>
                         <DrawerClose className="flex justify-start gap-4">
                          <Button variant="ghost" className="w-40">
                            Cancel
                          </Button>
                        </DrawerClose> 
                      </DrawerFooter> */}
                                        </DrawerContent>
                                    </Drawer>
                                </TableCell>
                                <TableCell>
                                    <Link
                                        to="/provider-datahub-catalog/$provider/$catalogId/dataset/$datasetId"
                                        params={{provider, catalogId: catalogId, datasetId: dataset.urn}}
                                    >
                                        <Button variant="link">
                                            See dataset
                                            <ArrowRight/>
                                        </Button>
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

export const Route = createFileRoute("/provider-datahub-catalog/$provider/$catalogId/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
});
