import {createFileRoute, Link} from "@tanstack/react-router";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import Heading from "shared/src/components/ui/heading.tsx";
import {useBusinessGetDatahubCatalogs} from "shared/src/data/business-queries.ts";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { Badge } from "shared/src/components/ui/badge";
import dayjs from "dayjs";
import { ArrowRight } from "lucide-react";


const RouteComponent = () => {
    const {data: datahubCatalogs} = useBusinessGetDatahubCatalogs();
    return (
        <div className="space-y-4">
            <div>
              <Heading level="h3" className="flex gap-2 items-center">Catalogs</Heading>
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

                                         <TableCell>                        <Badge variant="info">{datahubCatalog.urn.slice(4)}</Badge></TableCell>
                            <TableCell>{datahubCatalog.properties.name}</TableCell>
                            <TableCell>{datahubCatalog.properties.description}</TableCell>
                            <TableCell>
                             
                                    <Link
                                      
                                        to="/datahub-catalog/$catalogId"
                                        params={{catalogId: datahubCatalog.urn}}
                                    >
                                           <Button variant="link">
                                        See catalog
                                           <ArrowRight />
                                             </Button>
                                    </Link>
                                       
                           
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
