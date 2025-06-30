import { createFileRoute, Link  } from "@tanstack/react-router";
import dayjs from "dayjs";
import { useState } from "react";
import { ExternalLink } from "lucide-react";
import { Badge } from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";
import { RouteComponent as OfferForm } from "@/routes/contract-negotiation/offer";

import {
  Drawer,
  DrawerBody,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "@./../../shared/src/components/ui/drawer.tsx";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table";
import {
  useGetCatalogsById,
  useGetDataServicesByCatalogId,
  useGetDatasetsByCatalogId,
} from "shared/src/data/catalog-queries.ts";
import { Button, buttonVariants } from "shared/src/components/ui/button";
// Icons
import { ArrowRight } from "lucide-react";

const RouteComponent = () => {
  const { catalogId } = Route.useParams();
  const { data: catalog } = useGetCatalogsById(catalogId);
  const { data: datasets } = useGetDatasetsByCatalogId(catalogId);
  const { data: dataservices } = useGetDataServicesByCatalogId(catalogId);
  const [selectedDataset, setSelectedDataset] = useState();

  return (
    <div className="space-y-4 pb-4">
      <div>
        <Heading level="h5">Catalog info:</Heading>

        <List>
          <ListItem>
            <ListItemKey>Catalog title</ListItemKey>
            <p>{catalog.title}</p>
            
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog participant ID</ListItemKey>
            <Badge variant="info">
              {catalog.participantId.slice(9, 29) + "[...]"}
            </Badge>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog homepage</ListItemKey>
            <p>{catalog.homepage}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog creation date</ListItemKey>
            <ListItemDate>
              {dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}
            </ListItemDate>
          </ListItem>
        </List>
      </div>

      <div>
        <Heading level="h5">Datasets</Heading>

        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Dataset ID</TableHead>
              <TableHead>Title</TableHead>
              <TableHead>Provider ID</TableHead>
              <TableHead>Created at</TableHead>
              <TableHead>Actions</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {datasets.map((dataset) => (
              <TableRow key={dataset["@id"].slice(9, 29)}>
                <TableCell>
                  <Badge variant="info">
                    {" "}
                    {dataset["@id"].slice(9, 29) + "..."}
                  </Badge>
                </TableCell>
                <TableCell>{dataset.title}</TableCell>
                <TableCell>
                  <Badge variant="info">
                    {catalog.participantId.slice(9, 29) + "..."}
                  </Badge>
                </TableCell>
                <TableCell>
                  <ListItemDate>
                    {dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}
                  </ListItemDate>
                </TableCell>
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
                        {/* {console.log("datasetto", dataset)} */}

                        <OfferForm catalog={catalog} dataset={dataset}  />
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
                  <Link
                    to="/catalog/$catalogId/dataset/$datasetId"
                    params={{
                      catalogId: catalog["@id"],
                      datasetId: dataset["@id"],
                    }}
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
      <div>
        <Heading level="h5">Dataservices</Heading>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Dataservice Id</TableHead>
              <TableHead>Endpoint</TableHead>
              <TableHead>Endpoint Description</TableHead>
              <TableHead>Created at</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {dataservices.map((dataservice) => (
              <TableRow key={dataservice["@id"].slice(9, 29)}>
                <TableCell>
                  <Badge variant="info">
                    {dataservice["@id"].slice(9, 29) + "..."}
                  </Badge>
                </TableCell>
                <TableCell>{dataservice.endpointURL}</TableCell>
                <TableCell>{dataservice.endpointDescription}</TableCell>
                <TableCell>
                  <ListItemDate>
                    {" "}
                    {dayjs(dataservice.issued).format("DD/MM/YYYY - HH:mm")}
                  </ListItemDate>
                </TableCell>
                <TableCell>
                  <Link
                    to="/catalog/$catalogId/data-service/$dataserviceId"
                    params={{
                      catalogId: catalog["@id"],
                      dataserviceId: dataservice["@id"],
                    }}
                  >
                    <Button variant="link">
                      See dataservice
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

export const Route = createFileRoute("/catalog/$catalogId/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
