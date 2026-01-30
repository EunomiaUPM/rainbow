import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import { ArrowRight } from "lucide-react";
import {
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow,
  TableCell,
} from "shared/src/components/ui/table";
import { useGetCatalogs, useGetMainCatalogs } from "shared/src/data/catalog-queries.ts";
import Heading from "shared/src/components/ui/heading.tsx";
import { List, ListItem, ListItemKey, ListItemDate } from "shared/src/components/ui/list.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { Badge } from "shared/src/components/ui/badge";

const RouteComponent = () => {
  const { data: mainCatalog } = useGetMainCatalogs();
  const { data: catalogs } = useGetCatalogs(false);
  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Main Catalog with id
        <Badge variant="info" size="lg">
          {mainCatalog.id.slice(0, 20) + "[...]"}{" "}
        </Badge>
      </Heading>
      <div>
        <Heading level="h5">Main Catalog info: </Heading>
        <div className="gridColsLayout">
          <List className="text-sm">
            <ListItem>
              <ListItemKey>Catalog title</ListItemKey>
              <p>{mainCatalog?.dctTitle}</p>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog participant id</ListItemKey>
              <Badge variant="info">
                {" "}
                {mainCatalog.dspaceParticipantId.slice(0, 20) + "[...]"}{" "}
              </Badge>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog homepage</ListItemKey>
              <p>{mainCatalog.foafHomePage}</p>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog creation date</ListItemKey>
              <ListItemDate>
                {dayjs(mainCatalog.dctIssued).format("DD/MM/YYYY - HH:mm")}
              </ListItemDate>
            </ListItem>
          </List>
          <div className="filler"></div>
        </div>
      </div>

      <div>
        <Heading level="h5">Catalogs</Heading>
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Title</TableHead>
              <TableHead>Created at</TableHead>
              <TableHead>Catalog ID</TableHead>
              <TableHead>Provider ID</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {catalogs?.map((catalogItem) => (
              <TableRow key="urn:uuid:c4d4449d-a">
                <TableCell>
                  <p className="text-18">{catalogItem.dctTitle}</p>
                </TableCell>
                <TableCell>
                  <ListItemDate> 23/6/25 16:34 </ListItemDate>
                </TableCell>
                <TableCell>
                  {" "}
                  <Badge variant="info">{catalogItem.id.slice(0, 20) + "[...]"} </Badge>
                </TableCell>
                <TableCell>
                  {" "}
                  <Badge variant="info">
                    {catalogItem.dspaceParticipantId.slice(0, 20) + "[...]"}
                  </Badge>
                </TableCell>
                <TableCell>
                  <Link to="/catalog/$catalogId" params={{ catalogId: catalogItem.id }}>
                    <Button variant={"link"}>
                      See catalog
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

export const Route = createFileRoute("/catalog/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
