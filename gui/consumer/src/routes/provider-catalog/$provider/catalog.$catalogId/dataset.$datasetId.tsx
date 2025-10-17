import {createFileRoute, Link} from "@tanstack/react-router";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import {ArrowRight} from "lucide-react";
import {
  useGetBypassDatasetById,
  useGetBypassDistributionsByDatasetId,
} from "../../../../../../shared/src/data/catalog-bypass-queries.ts";
import {
  useGetBypassPoliciesByDatasetId
} from "../../../../../../shared/src/data/policy-bypass-queries.ts";
import {Button} from "shared/src/components/ui/button.tsx";
import Heading from "shared/src/components/ui/heading";
import {List, ListItem, ListItemDate, ListItemKey} from "shared/src/components/ui/list.tsx";
import {Badge} from "shared/src/components/ui/badge";
import {PolicyWrapperShow} from "shared/src/components/PolicyWrapperShow.tsx";
import {Dialog, DialogTrigger} from "shared/src/components/ui/dialog.tsx";
import {
  ContractNegotiationNewRequestDialog
} from "shared/src/components/ContractNegotiationNewRequestDialog.tsx";

function RouteComponent() {
  const {provider, catalogId, datasetId} = Route.useParams();
  const {data: dataset} = useGetBypassDatasetById(provider, datasetId);
  const {data: distributions} = useGetBypassDistributionsByDatasetId(provider, datasetId);
  const {data: policies} = useGetBypassPoliciesByDatasetId(provider, datasetId);
  

  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Dataset with id
        <Badge variant="info" size="lg">
          {" "}
          {dataset["@id"].slice(9, 29) + "[...]"}
        </Badge>
      </Heading>
      <div className="gridColsLayout">
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Dataset title</ListItemKey>
            <p>{dataset.title}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog creation date</ListItemKey>
            <ListItemDate>{dayjs(dataset.issued).format("DD/MM/YYYY - HH:mm")}</ListItemDate>
          </ListItem>
        </List>
      </div>

      <div>
        <Heading level="h5">Distributions</Heading>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Distribution Id</TableHead>
              <TableHead>Distribution Title</TableHead>
              <TableHead>Created at</TableHead>
              <TableHead>Associated Data service</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {distributions.map((distribution) => (
              <TableRow key={distribution["@id"].slice(0, 20)}>
                <TableCell>
                  <Badge variant="info">{distribution["@id"].slice(9, 29) + "[...]"}</Badge>
                </TableCell>
                <TableCell>{distribution.title ? distribution.title : "undefined"}</TableCell>
                <TableCell>
                  <ListItemDate>
                    {dayjs(distribution.issued).format("DD/MM/YYYY - HH:mm")}
                  </ListItemDate>
                </TableCell>
                <TableCell>
                  <Link
                    to="/provider-catalog/$provider/catalog/$catalogId/data-service/$dataserviceId"
                    params={{
                      catalogId: catalogId,
                      dataserviceId: distribution.accessService["@id"],
                    }}
                  >
                    <Button variant="link">
                      See dataservice
                      <ArrowRight/>
                    </Button>
                  </Link>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      <div className="h-2">
        <div className="flex flex-row justify-between items-center">
          <Heading level="h5" className="mb-2">
            ODRL Policies
          </Heading>
        </div>
        <div className="grid grid-cols-2 gap-4">
          {policies.map((policy) => (
            <div className="flex flex-col gap-2">
              <PolicyWrapperShow
                policy={policy}
                datasetId={datasetId}
                catalogId={catalogId}
                participant={provider}
                datasetName={""}
              />
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="default" size="sm" className="self-start">
                    Request Contract Negotiation
                  </Button>
                </DialogTrigger>
                <ContractNegotiationNewRequestDialog
                  policy={policy}
                  catalogId={catalogId}
                  datasetId={datasetId}
                  participantId={provider}
                />
              </Dialog>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export const Route = createFileRoute(
  "/provider-catalog/$provider/catalog/$catalogId/dataset/$datasetId",
)({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
