import {createFileRoute, Link} from "@tanstack/react-router";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import Heading from "shared/src/components/ui/heading.tsx";
import {Button,} from "shared/src/components/ui/button.tsx";
import {Input,} from "shared/src/components/ui/input.tsx";
import {useBusinessGetDatahubCatalogs} from "shared/src/data/business-queries.ts";

const RouteComponent = () => {
    const {data: datahubCatalogs} = useBusinessGetDatahubCatalogs();
    return (
        <div className="space-y-4">
            <div>
                <Heading level="h5">Catalogs</Heading>
                <div className='pb-3 w-3/5'>
                    <Input type="search"></Input>
                </div>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Id</TableHead>
                            <TableHead>Title</TableHead>
                            <TableHead>Description</TableHead>
                            <TableHead>Link</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {datahubCatalogs.map(datahubCatalog => (<TableRow key="urn:uuid:c4d4449d-a">

                            <TableCell>{datahubCatalog.urn}</TableCell>
                            <TableCell>{datahubCatalog.properties.name}</TableCell>
                            <TableCell>{datahubCatalog.properties.description}</TableCell>
                            <TableCell>
                                <Button>
                                    <Link
                                        to="/datahub-catalog/$catalogId"
                                        params={{catalogId: datahubCatalog.urn}}
                                    >
                                        See catalog
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

export const Route = createFileRoute("/datahub-catalog/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
});
