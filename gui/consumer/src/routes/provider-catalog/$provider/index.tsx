import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table";
import Heading from "shared/src/components/ui/heading.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
import { Button } from "@/../../shared/src/components/ui/button.tsx";
import { Input } from "@/../../shared/src/components/ui/input.tsx";
import { useGetBypassCatalogs } from "shared/src/data/catalog-bypass-queries.ts";
import { Badge } from "shared/src/components/ui/badge";
import { ArrowRight } from "lucide-react";

const RouteComponent = () => {
  const { provider } = Route.useParams();
  const { data: catalogs } = useGetBypassCatalogs(provider);
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
        <div className="gridColsLayout">
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
              <p>{dayjs(catalogs.issued).format("DD/MM/YYYY - HH:mm")}</p>
            </ListItem>
          </List>
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
            {catalogs.catalog.map((catalogItem) => (
              <TableRow key="urn:uuid:c4d4449d-a">
                <TableCell>
                  {" "}
                  <p className="text-18">{catalogItem.title}</p>
                </TableCell>
                <TableCell>
                  <p className="text-gray-400"> 23/6/25 16:34</p>
                </TableCell>

                <TableCell>
                  <Badge variant="info">
                    {catalogItem["@id"].slice(9, 29) + "[...]"}
                  </Badge>
                </TableCell>
                <TableCell>
                  <Badge variant="info">
                    {catalogItem.participantId.slice(9, 29) + "[...]"}
                  </Badge>
                </TableCell>

                <TableCell>
                  <Link
                    to="/provider-catalog/$provider/catalog/$catalogId"
                    params={{
                      provider: provider,
                      catalogId: catalogItem["@id"],
                    }}
                  >
                    <Button variant="link">
                      See catalog
                      <ArrowRight />
                    </Button>
                  </Link>
                </TableCell>
              </TableRow>
            ))}

            {/* {catalogs.catalog.map((catalog) => (
                            <TableRow key={catalog["@id"].slice(0, 20)}>
                                <p>
                                    {catalog["@id"].slice(0, 20) + "..."}
                                </p>
                                <p>
                                    {catalog.title?.slice(0, 20) + "..."}
                                </p>
                                <p>{catalog.participantId}</p>
                                <p>
                                    {dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}
                                </p>
                                <p>
                                    <Link
                                        to="/catalog/$catalogId"
                                        params={{catalogId: catalog["@id"]}}
                                    >
                                        <ExternalLink size={12} className="text-pink-600"/>
                                    </Link>
                                </p>
                            </TableRow>
                        ))} */}
          </TableBody>
        </Table>
      </div>
    </div>
  );
};

export const Route = createFileRoute("/provider-catalog/$provider/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
