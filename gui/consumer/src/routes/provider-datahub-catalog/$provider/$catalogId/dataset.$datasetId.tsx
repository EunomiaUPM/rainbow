import { createFileRoute } from "@tanstack/react-router";
import Heading from "shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list";
import { useGetDatahubBypassDatasetById } from "shared/src/data/catalog-datahub-bypass-queries.ts";
import { useGetBypassPoliciesByDatasetId } from "shared/src/data/policy-bypass-queries.ts";
import { PolicyWrapperShow } from "shared/src/components/PolicyWrapperShow.tsx";
import { Dialog, DialogTrigger } from "shared/src/components/ui/dialog.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { ContractNegotiationNewRequestDialog } from "shared/src/components/ContractNegotiationNewRequestDialog.tsx";

function RouteComponent() {
  const { provider, catalogId, datasetId } = Route.useParams();
  const { data: dataset } = useGetDatahubBypassDatasetById(provider, datasetId);
  const { data: policies } = useGetBypassPoliciesByDatasetId(provider, datasetId);

  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Dataset
        <Badge variant="info" size="lg">
          {" "}
          {dataset.name}
        </Badge>
      </Heading>

      <List className="text-sm w-2/3">
        {dataset.custom_properties.map((property) => (
          <ListItem key={property[0]}>
            <ListItemKey className="basis-[30%] text-sky-300">{property[0]}</ListItemKey>
            <p className="text-gray-300/90">{property[1]}</p>
          </ListItem>
        ))}
      </List>
      <div className="h-2"></div>
      <div className=" flex flex-row  justify-between items-center">
        <Heading level="h5" className="mb-0">
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
  );
}

export const Route = createFileRoute(
  "/provider-datahub-catalog/$provider/$catalogId/dataset/$datasetId",
)({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
