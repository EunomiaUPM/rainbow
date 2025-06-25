import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import { ArrowRight, ExternalLink } from "lucide-react";
import {
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow,
  TableCell,
} from "shared/src/components/ui/table";
import { useGetCatalogs } from "shared/src/data/catalog-queries.ts";
import Heading from "shared/src/components/ui/heading.tsx";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { Badge } from "shared/src/components/ui/badge";

const RouteComponent = () => {
  const { data: catalogs } = useGetCatalogs();
  // console.log(catalogs, " catalogsss");
  return (
    <div className="space-y-4 pb-4">
      {/* <h1 className="text-xl font-bold">Catalogs</h1> */}
      <Heading level="h3" className="flex gap-2 items-center">
        Main Catalog with id
        <Badge variant="info" size="lg">
          {catalogs["@id"].slice(9, 29) + "[...]"}{" "}
        </Badge>
      </Heading>
      <div>
        <Heading level="h5">Main Catalog info: </Heading>
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Catalog title</ListItemKey>
            <p>{catalogs.title}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog participant id</ListItemKey>
            <Badge variant="info">
              {" "}
              {catalogs.participantId.slice(9, 29) + "[...]"}{" "}
            </Badge>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog homepage</ListItemKey>
            <p>{catalogs.homepage}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog creation date</ListItemKey>
            <ListItemDate>
              {dayjs(catalogs.issued).format("DD/MM/YYYY - HH:mm")}
            </ListItemDate>
          </ListItem>
        </List>
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
              <TableHead>Created at </TableHead>
              <TableHead>Catalog ID</TableHead>
              <TableHead>Provider ID</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {catalogs?.catalog?.map((catalogItem) => (
              <TableRow key="urn:uuid:c4d4449d-a">
                <TableCell>
                  <p className="text-18">{catalogItem.title}</p>
                </TableCell>
                <TableCell>
                  <ListItemDate> 23/6/25 16:34 </ListItemDate>
                </TableCell>
                <TableCell>
                  {" "}
                  <Badge variant="info">
                    {catalogItem["@id"].slice(9, 29) + "[...]"}{" "}
                  </Badge>
                </TableCell>
                <TableCell>
                  {" "}
                  <Badge variant="info">
                    {catalogItem.participantId.slice(9, 29) + "[...]"}
                  </Badge>
                </TableCell>
                <TableCell>
                  <Link
                    to="/catalog/$catalogId"
                    params={{ catalogId: catalogItem["@id"] }}
                  >
                    <Button variant={'link'}>See catalog
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
