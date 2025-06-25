import { createFileRoute, Link } from "@tanstack/react-router";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import { ExternalLink } from "lucide-react";
import {
  useGetBypassDatasetById,
  useGetBypassDistributionsByDatasetId,
} from "../../../../../../shared/src/data/catalog-bypass-queries.ts";
import { useGetBypassPoliciesByDatasetId } from "../../../../../../shared/src/data/policy-bypass-queries.ts";
import { Input } from "shared/src/components/ui/input";
import { Button } from "shared/src/components/ui/button.tsx";
import Heading from "shared/src/components/ui/heading";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list.tsx";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/ui/policyComponent.tsx";
import { ArrowRight } from "lucide-react";

function RouteComponent() {
  const { provider, catalogId, datasetId } = Route.useParams();
  const { data: dataset } = useGetBypassDatasetById(provider, datasetId);
  const { data: distributions } = useGetBypassDistributionsByDatasetId(
    provider,
    datasetId
  );
  const { data: policies } = useGetBypassPoliciesByDatasetId(
    provider,
    datasetId
  );

  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Dataset with id
        <Badge variant="info" size="lg">
          {" "}
          {dataset["@id"].slice(9, 29) + "[...]"}
        </Badge>
      </Heading>
      <div>
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Dataset title</ListItemKey>
            <p>{dataset.title}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog creation date</ListItemKey>
            <ListItemDate>
              {dayjs(dataset.issued).format("DD/MM/YYYY - HH:mm")}
            </ListItemDate>
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
                  <Badge variant="info">
                    {distribution["@id"].slice(9, 29) + "[...]"}
                  </Badge>
                </TableCell>
                <TableCell>
                  {distribution.title ? distribution.title : "undefined"}
                </TableCell>
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
                      <ArrowRight />
                    </Button>
                  </Link>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

    
          <Heading level="h5"> ODRL Policies </Heading>
          <div className="container-policies flex flex-wrap gap-4">
          {policies.map((policy) => (
            <List className=" border border-white/30 bg-white/10 px-4 py-2 rounded-md justify-start">
              <div className="flex">
                <Heading level="h5" className="flex gap-3">
                  <div>Policy with ID</div>
                  <Badge variant="info" className="h-6">
                    {policy["@id"].slice(9, 29) + "[...]"}
                  </Badge>
                </Heading>
              </div>
              <ListItem>
                <ListItemKey>Policy Target</ListItemKey>
                <p>{policy["@type"]}</p>
              </ListItem>

              <ListItem>
                <ListItemKey> Profile</ListItemKey>
                <p className="whitespace-normal">
                  {" "}
                  {JSON.stringify(policy.profile)}
                </p>
              </ListItem>
              <ListItem>
                <ListItemKey> Target</ListItemKey>
                <p> {policy.target.slice(9)}</p>
              </ListItem>
              <div className="h-5"></div>
              <Heading level="h6"> ODRL CONTENT</Heading>

              <div className="flex flex-col gap-2 mb-2">
                <PolicyComponent
                  policyItem={policy.permission}
                  variant={"permission"}
                />
                <PolicyComponent
                  policyItem={policy.obligation}
                  variant={"obligation"}
                />

                <PolicyComponent
                  policyItem={policy.prohibition}
                  variant={"prohibition"}
                />
              </div>
            </List>
          ))}
        </div>
      </div>

  );
}

export const Route = createFileRoute(
  "/provider-catalog/$provider/catalog/$catalogId/dataset/$datasetId"
)({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
