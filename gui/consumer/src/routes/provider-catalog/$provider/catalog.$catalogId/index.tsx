import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import { ExternalLink } from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import {
  useGetBypassCatalogsById,
  useGetBypassDataServicesByCatalogId,
  useGetBypassDatasetsByCatalogId,
} from "shared/src/data/catalog-bypass-queries.ts";
import { Badge } from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list";
import { Button, buttonVariants } from "shared/src/components/ui/button";
import { ArrowRight } from "lucide-react";

const RouteComponent = () => {
  const { provider, catalogId } = Route.useParams();
  const { data: catalog } = useGetBypassCatalogsById(provider, catalogId);
  const { data: datasets } = useGetBypassDatasetsByCatalogId(
    provider,
    catalogId
  );
  const { data: dataservices } = useGetBypassDataServicesByCatalogId(
    provider,
    catalogId
  );

  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Catalog
        <Badge variant="info" size="lg">
          {catalog["@id"].slice(9, 29) + "[...]"}
        </Badge>
      </Heading>

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
                  <Button size="sm" variant="outline">
                    + Request dataset
                  </Button>
                </TableCell>
                <TableCell>
                  <Link
                    to="/provider-catalog/$provider/catalog/$catalogId/dataset/$datasetId"
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
              <TableHead>Actions</TableHead>
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
                  <Button size="sm" variant="outline">
                    + Request dataservice
                  </Button>
                </TableCell>
                <TableCell>
                  <Link
                    to="/provider-catalog/$provider/catalog/$catalogId/data-service/$dataserviceId"
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

export const Route = createFileRoute(
  "/provider-catalog/$provider/catalog/$catalogId/"
)({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
