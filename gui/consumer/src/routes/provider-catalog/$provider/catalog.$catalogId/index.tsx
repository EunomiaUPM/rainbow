import {createFileRoute, Link} from "@tanstack/react-router";
import dayjs from "dayjs";
import {ArrowRight} from "lucide-react";
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
  useGetBypassDatasetsByCatalogId,
} from "shared/src/data/catalog-bypass-queries.ts";
import {Badge} from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";
import {List, ListItem, ListItemDate, ListItemKey} from "shared/src/components/ui/list";
import {Button} from "shared/src/components/ui/button";
import {
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger
} from "shared/src/components/ui/drawer.tsx";
import {RainbowRequestDrawer} from "@/components/RainbowRequestDrawer.tsx";

const RouteComponent = () => {
  const {provider, catalogId} = Route.useParams();
  const {data: catalog} = useGetBypassCatalogsById(provider, catalogId);
  const {data: datasets} = useGetBypassDatasetsByCatalogId(provider, catalogId);

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
        <div className="gridColsLayout">
          <List>
            <ListItem>
              <ListItemKey>Catalog title</ListItemKey>
              <p>{catalog.title}</p>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog participant ID</ListItemKey>
              <Badge variant="info">{catalog.participantId.slice(9, 29) + "[...]"}</Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog homepage</ListItemKey>
              <p>{catalog.homepage}</p>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog creation date</ListItemKey>
              <ListItemDate>{dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}</ListItemDate>
            </ListItem>
          </List>
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
              <TableRow key={dataset["@id"].slice(9, 29)}>
                <TableCell>
                  <Badge variant="info"> {dataset["@id"].slice(9, 29) + "..."}</Badge>
                </TableCell>
                <TableCell>{dataset.title}</TableCell>
                <TableCell>
                  <Badge variant="info">{catalog.participantId.slice(9, 29) + "..."}</Badge>
                </TableCell>
                <TableCell>
                  <ListItemDate>{dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}</ListItemDate>
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
                        <RainbowRequestDrawer
                          catalogId={catalogId}
                          datasetId={dataset["@id"]}
                          participantId={provider}
                        />
                      </DrawerBody>
                    </DrawerContent>
                  </Drawer>
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

export const Route = createFileRoute("/provider-catalog/$provider/catalog/$catalogId/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
