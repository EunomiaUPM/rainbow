import { createFileRoute, Link } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import dayjs from "dayjs";
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
} from "@./../../shared/src/components/ui/drawer";
import { List, ListItem, ListItemDate, ListItemKey } from "shared/src/components/ui/list";
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
import { Button } from "shared/src/components/ui/button";
// Icons
import { ArrowRight, Plus } from "lucide-react";

const RouteComponent = () => {
  const { catalogId } = Route.useParams();
  const { data: catalog } = useGetCatalogsById(catalogId);
  const { data: datasets } = useGetDatasetsByCatalogId(catalogId);
  const { data: dataservices } = useGetDataServicesByCatalogId(catalogId);

  return (
    <div className="space-y-4 pb-4">
      <div>
        <Heading level="h5">Catalog info:</Heading>
        <div className="gridColsLayout">
          <List>
            <ListItem>
              <ListItemKey>Catalog title</ListItemKey>
              <p>{catalog.dctTitle}</p>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog participant ID</ListItemKey>
              <Badge variant="info">{formatUrn(catalog.dspaceParticipantId)} </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog homepage</ListItemKey>
              <p>{catalog.foafHomePage}</p>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog creation date</ListItemKey>
              <ListItemDate>{dayjs(catalog.dctIssued).format("DD/MM/YYYY - HH:mm")}</ListItemDate>
            </ListItem>
          </List>
          <div className="filler"></div>
        </div>
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
              <TableRow key={formatUrn(dataset.id)}>
                <TableCell>
                  <Badge variant="info"> {formatUrn(dataset.id)}</Badge>
                </TableCell>
                <TableCell>{dataset.dctTitle}</TableCell>
                <TableCell>
                  <Badge variant="info">
                    {formatUrn(catalog.dspaceParticipantId)}{" "}
                  </Badge>
                </TableCell>
                <TableCell>
                  <ListItemDate>
                    {dayjs(dataset.dctIssued).format("DD/MM/YYYY - HH:mm")}
                  </ListItemDate>
                </TableCell>
                <TableCell>
                  <Drawer direction={"right"}>
                    <DrawerTrigger>
                      <Button variant="outline" size="sm">
                        <Plus />
                        Offer dataset
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
                        <OfferForm catalog={catalog} dataset={dataset} />
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
                      catalogId: catalog.id,
                      datasetId: dataset.id,
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
              <TableHead>Created at</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {dataservices.map((dataservice) => (
              <TableRow key={formatUrn(dataservice.id)}>
                <TableCell>
                  <Badge variant="info">{formatUrn(dataservice.id)}</Badge>
                </TableCell>

                <TableCell>
                  <ListItemDate>
                    {" "}
                    {dayjs(dataservice.dctIssued).format("DD/MM/YYYY - HH:mm")}
                  </ListItemDate>
                </TableCell>
                <TableCell>
                  <Link
                    to="/catalog/$catalogId/data-service/$dataserviceId"
                    params={{
                      catalogId: catalog.id,
                      dataserviceId: dataservice.id,
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
