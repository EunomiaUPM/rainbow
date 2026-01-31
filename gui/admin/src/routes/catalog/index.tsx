import { createFileRoute, Link } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
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
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";
import { InfoGrid } from "shared/src/components/layout/InfoGrid";

const RouteComponent = () => {
  const { data: mainCatalog } = useGetMainCatalogs();
  const { data: catalogs } = useGetCatalogs(false);
  return (
    <PageLayout>
      <PageHeader
        title="Main Catalog with id"
        badge={<Badge variant="info" size="lg">{formatUrn(mainCatalog.id)}</Badge>}
      />
      <InfoGrid>
        <PageSection title="Main Catalog info: ">
          <List className="text-sm">
            <ListItem>
              <ListItemKey>Catalog title</ListItemKey>
              <p>{mainCatalog?.dctTitle}</p>
            </ListItem>
            <ListItem>
              <ListItemKey>Catalog participant id</ListItemKey>
              <Badge variant="info">
                {" "}
                {formatUrn(mainCatalog.dspaceParticipantId)}{" "}
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
        </PageSection>
      </InfoGrid>

      <PageSection title="Catalogs">
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
              <TableRow key={formatUrn(catalogItem.id)}>
                <TableCell>
                  <p className="text-18">{catalogItem.dctTitle}</p>
                </TableCell>
                <TableCell>
                  <ListItemDate>
                    {dayjs(catalogItem.dctIssued).format("DD/MM/YYYY - HH:mm")}
                  </ListItemDate>
                </TableCell>
                <TableCell>
                  {" "}
                  <Badge variant="info">{formatUrn(catalogItem.id)} </Badge>
                </TableCell>
                <TableCell>
                  {" "}
                  <Badge variant="info">
                    {formatUrn(catalogItem.dspaceParticipantId)}
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
      </PageSection>
    </PageLayout>
  );
};

/**
 * Route for listing catalogs.
 */
export const Route = createFileRoute("/catalog/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
