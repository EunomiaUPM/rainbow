import {createFileRoute, Link} from "@tanstack/react-router";
import {useState} from "react";
import {useGetDatahubDatasetsByCatalogId} from "shared/src/data/datahub-catalog-queries";
import Heading from "shared/src/components/ui/heading";
import {Input} from "shared/src/components/ui/input";
import {Badge} from "shared/src/components/ui/badge";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {Button} from "shared/src/components/ui/button";
import {
    Drawer,
    DrawerBody,
    DrawerContent,
    DrawerHeader,
    DrawerTitle,
    DrawerTrigger,
} from "shared/src/components/ui/drawer";
import {OfferDrawer} from "@/components/OfferDrawer";
import {ArrowRight} from "lucide-react";

const RouteComponent = () => {
    const [openedDrawerId, setOpenedDrawerId] = useState<string | null>(null);
    const {catalogId} = Route.useParams();
    const {data: datasets} = useGetDatahubDatasetsByCatalogId(catalogId);
    const closeDrawer = () => setOpenedDrawerId(null);

    return (
        <div className="space-y-4 pb-4">
            <div>
                <Heading level="h5">Datasets</Heading>
                <div className="pb-3 w-3/5">
                    <Input type="search"></Input>
                </div>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow className="">
                            <TableHead>Name</TableHead>
                            <TableHead>ETL system</TableHead>
                            <TableHead>Description</TableHead>
                            <TableHead>Glossary</TableHead>
                            <TableHead>Offer</TableHead>
                            <TableHead>Link</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {datasets.map((dataset) => (
                            <TableRow key={dataset.urn}>
                                <TableCell className="min-w-[196px] max-w-[300px] break-all">
                                    {dataset.name}
                                </TableCell>
                                <TableCell>
                                    <Badge variant="info">{dataset.platform.name}</Badge>
                                </TableCell>
                                <TableCell className="min-w-[200px]">
                                    <p className="text-xs">
                                        {" "}
                                        {dataset.description.length > 70
                                            ? dataset.description.slice(0, 70) + "..."
                                            : dataset.description}{" "}
                                    </p>
                                </TableCell>
                                <TableCell>
                                    <Badge className="default">
                                        {dataset.glossary_terms.map((m) => (
                                            <span key={m.urn}>{m.glossaryTermInfo.name}</span>
                                        ))}
                                    </Badge>
                                </TableCell>
                                <TableCell>
                                    <Drawer direction={"right"}
                                            open={openedDrawerId === dataset.urn} // Se abre solo si su ID coincide
                                            onOpenChange={(isOpen) => {
                                                if (!isOpen) {
                                                    setOpenedDrawerId(null); // Si el usuario lo cierra, actualiza el estado
                                                }
                                            }}>
                                        <DrawerTrigger asChild={true}>
                                            <Button variant="outline" size="sm"
                                                    onClick={() => setOpenedDrawerId(dataset.urn)}>
                                                + Offer dataset
                                            </Button>
                                        </DrawerTrigger>
                                        <DrawerContent>
                                            <DrawerHeader>
                                                <DrawerTitle asChild={true}>
                                                    <Heading level="h5" className="text-current">
                                                        New Contract Negotiation Offer
                                                    </Heading>
                                                </DrawerTitle>
                                            </DrawerHeader>
                                            <DrawerBody className="items-start">
                                                <OfferDrawer
                                                    catalogId={catalogId}
                                                    datasetId={dataset.urn}
                                                    closeDrawer={closeDrawer}
                                                />
                                            </DrawerBody>
                                        </DrawerContent>
                                    </Drawer>
                                </TableCell>
                                <TableCell>
                                    <Link
                                        to="/datahub-catalog/$catalogId/dataset/$datasetId"
                                        params={{catalogId: catalogId, datasetId: dataset.urn}}
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

export const Route = createFileRoute("/datahub-catalog/$catalogId/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
});
