import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import { useState } from "react";
import { useGetDatahubDatasetsByCatalogId } from "shared/src/data/datahub-catalog-queries";
import Heading from "shared/src/components/ui/heading.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { Badge } from "shared/src/components/ui/badge";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import {
  Drawer,
  DrawerBody,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "shared/src/components/ui/drawer";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list";
import { useGetParticipants } from "shared/src/data/participant-queries.ts";
import { OfferDrawer } from "@/components/OfferDrawer.tsx";
import { ArrowRight, Plus } from "lucide-react";

const RouteComponent = () => {
   const [open, setOpen] = useState(false)
  const { catalogId } = Route.useParams();
  const { data: datasets } = useGetDatahubDatasetsByCatalogId(catalogId);

  const { data: participants } = useGetParticipants();

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
              {/* <TableHead>Id</TableHead> */}
              <TableHead>Name</TableHead>
              <TableHead>ETL system</TableHead>
              <TableHead>Description</TableHead>
              {/* <TableHead>Tags</TableHead> */}
              <TableHead>Glossary</TableHead>
              <TableHead>Offer</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {datasets.map((dataset) => (
              <TableRow key="" >
                {/* <TableCell>{dataset.urn.slice(19,25)}...</TableCell> */}
                <TableCell className="min-w-[196px] max-w-[300px] break-all">{dataset.name}</TableCell>
                <TableCell>
                    <Badge variant="info">
                    {dataset.platform.name}
                    </Badge>
                    </TableCell>
                <TableCell className="min-w-[200px]">
                  <p className="text-xs"> {dataset.description.length > 70
    ? dataset.description.slice(0, 70) + "..."
    : dataset.description} </p>
                    </TableCell>
                {/* <TableCell>{dataset.tag_names.join(", ")}</TableCell> */}
                <TableCell>
                    <Badge >
                  {dataset.glossary_terms.map((m) => (
                    <span>{m.glossaryTermInfo.name}</span>
                  ))}
                  </Badge>
                </TableCell>
                <TableCell>
                  <Drawer direction={"right"}>
                    <DrawerTrigger>
                      <Button variant="outline" size="sm">
                        {/* <Plus/> */}
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
                      <DrawerBody className="items-start">
                        <OfferDrawer
                          catalogId={catalogId}
                          datasetId={dataset.urn}
                           closeDrawer={() => setOpen(false)}
                        />
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
                    to="/datahub-catalog/$catalogId/dataset/$datasetId"
                    params={{ catalogId: catalogId, datasetId: dataset.urn }}
                  >
                    <Button variant="link">
                      See dataset
                      <ArrowRight />
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
