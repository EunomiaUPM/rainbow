import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import { Input } from "shared/src/components/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import { Button } from "shared/src/components/ui/button";
import { useGetBusinessDatahubDatasetsByCatalogId } from "shared/src/data/business-queries.ts";
import { ArrowRight, Plus } from "lucide-react";
import { Badge } from "shared/src/components/ui/badge";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list";
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

const RouteComponent = () => {
  const { catalogId } = Route.useParams();
  const { data: datasets } =
    useGetBusinessDatahubDatasetsByCatalogId(catalogId);

  return (
    <div className="space-y-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Catalog
        <Badge variant="info" size="lg">
          {catalogId.slice(14, 29)}
        </Badge>
      </Heading>
      <div>
        <Heading level="h5">Datasets</Heading>
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              {/* <TableHead>Id</TableHead> */}
              <TableHead>Name</TableHead>
              <TableHead>ETL system</TableHead>
              <TableHead>Description</TableHead>
              <TableHead>Glossary</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {datasets.map((dataset) => (
              <TableRow key="urn:uuid:c4d4449d-a">
                {/* <TableCell className="">{dataset.urn.slice(0, 15)}...</TableCell> */}
                <TableCell className="min-w-[196px] max-w-[300px] break-all">
                  {dataset.name}
                </TableCell>
                <TableCell>
                  <Badge variant="info">{dataset.platform.name}</Badge>
                </TableCell>
                <TableCell className="min-w-[200px]">
                  <p className="text-xs">
                    {" "}
                    {dataset.description.length > 70
                      ? dataset.description.slice(0, 70) + "..."
                      : dataset.description}{" "}
                  </p>
                </TableCell>
                {/* <TableCell>{dataset.tag_names.join(", ")}</TableCell> */}
                <TableCell>
                  <Badge className="default">
                    {dataset.glossary_terms.map((m) => (
                      <span>{m.glossaryTermInfo.name}</span>
                    ))}
                  </Badge>
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
