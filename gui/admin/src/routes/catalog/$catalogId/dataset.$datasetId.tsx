import { createFileRoute, Link } from "@tanstack/react-router";
import {
  useGetDatasetById,
  useGetDistributionsByDatasetId,
  getDatasetByIdOptions,
  getDistributionsByDatasetIdOptions,
} from "shared/src/data/catalog-queries.ts";
import { DataTable } from "shared/src/components/DataTable";
import { FormatDate } from "shared/src/components/ui/format-date";
import { ArrowRight, Plus } from "lucide-react";
import { useGetPoliciesByDatasetId } from "shared/src/data/policy-queries.ts";
import { SubmitHandler } from "react-hook-form";
import { useEffect } from "react";
import { Button } from "shared/src/components/ui/button.tsx";
import { usePostNewPolicyInDataset } from "shared/src/data/catalog-mutations.ts";
import { formatUrn } from "shared/src/lib/utils";
import Heading from "shared/src/components/ui/heading";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { InfoGrid } from "shared/src/components/layout/InfoGrid";
import { PageSection } from "shared/src/components/layout/PageSection";
import { InfoList } from "shared/src/components/ui/info-list";
import { Badge } from "shared/src/components/ui/badge";
import {
  Drawer,
  DrawerContent,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "shared/src/components/ui/drawer";
import { PolicyWrapperNew } from "shared/src/components/PolicyWrapperNew.tsx";
import { PolicyWrapperShow } from "shared/src/components/PolicyWrapperShow.tsx";
import { useContext, useState } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext.tsx";

type Inputs = {
  odrl: string;
};

function RouteComponent() {
  const { catalogId, datasetId } = Route.useParams();
  const { data: dataset } = useGetDatasetById(datasetId);
  const { data: distributions } = useGetDistributionsByDatasetId(datasetId);
  const { data: policies } = useGetPoliciesByDatasetId(dataset.id);
  const [open, setOpen] = useState(false);
  const { mutateAsync: createPolicyAsync } = usePostNewPolicyInDataset();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const participant = {
    participant_type: "Provider",
  };
  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    await createPolicyAsync({
      api_gateway,
      datasetId,
      content: {
        offer: JSON.stringify(data),
      },
    });
    setOpen(false);
  };

  return (
    <PageLayout>
      <PageHeader
        title="Dataset with id"
        badge={<Badge variant="info" size="lg">{formatUrn(dataset.id)}</Badge>}
      />
      <InfoGrid>
        <PageSection>
          <InfoList
            items={[
              { label: "Dataset title", value: dataset.dctTitle },
              {
                label: "Catalog creation date",
                value: { type: "custom", content: <FormatDate date={dataset.dctIssued} /> },
              },
            ]}
          />
        </PageSection>
      </InfoGrid>

      <PageSection title="Distributions">
        <DataTable
          className="text-sm"
          data={distributions}
          keyExtractor={(d) => d.id}
          columns={[
            {
              header: "Distribution Id",
              cell: (d) => <Badge variant="info">{formatUrn(d.id)}</Badge>,
            },
            {
              header: "Distribution Title",
              accessorKey: "dctTitle",
              cell: (d) => d.dctTitle ?? "undefined",
            },
            {
              header: "Created at",
              cell: (d) => <FormatDate date={d.dctIssued} />,
            },
            {
              header: "Associated Data service",
              cell: (d) => (
                <div className="flex gap-2">
                  <Link
                    to="/catalog/$catalogId/distribution-connector/$distributionId"
                    params={{
                      catalogId: catalogId,
                      distributionId: d.id,
                    }}
                  >
                    <Button variant="link">
                      See connector instance
                      <ArrowRight />
                    </Button>
                  </Link>
                  <Link
                    to="/catalog/$catalogId/data-service/$dataserviceId"
                    params={{
                      catalogId: catalogId,
                      dataserviceId: d.dcatAccessService,
                    }}
                  >
                    <Button variant="link">
                      See dataservice
                      <ArrowRight />
                    </Button>
                  </Link>
                </div>
              ),
            },
          ]}
        />
      </PageSection>

      <div className=" flex flex-row mb-2 items-center">
        <Heading level="h5" className="mb-0">
          {" "}
          ODRL Policies{" "}
        </Heading>
        <Drawer direction={"right"} open={open} onOpenChange={(open) => setOpen(open)}>
          <DrawerTrigger>
            <Button variant="default" size="sm" className="mb-1 ml-3">
              Add ODRL policy
              <Plus className="" />
            </Button>
          </DrawerTrigger>
          <DrawerContent>
            <DrawerHeader className="px-8">
              <DrawerTitle>
                <Heading level="h4" className="text-curren mb-0 ">
                  New ODRL Policy
                </Heading>
                <div className="font-normal text-brand-sky">
                  for Dataset
                  <Badge variant="info" size="sm" className="ml-2">
                    {formatUrn(dataset.id)}
                  </Badge>
                </div>
              </DrawerTitle>
            </DrawerHeader>
            <PolicyWrapperNew onSubmit={onSubmit} />
          </DrawerContent>
        </Drawer>
      </div>

      <div className="grid grid-cols-2 gap-4">
        {policies &&
          policies.map((policy) => (
            <PolicyWrapperShow
              key={policy.id}
              policy={policy}
              participant={participant}
              datasetId={dataset.id}
              catalogId={undefined}
              datasetName={dataset.dctTitle}
            />
          ))}
      </div>
    </PageLayout>
  );
}

/**
 * Route for displaying dataset details.
 */
export const Route = createFileRoute("/catalog/$catalogId/dataset/$datasetId")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
  loader: async ({ context: { queryClient, api_gateway }, params: { datasetId } }) => {
    if (!api_gateway) return;
    await queryClient.ensureQueryData(getDatasetByIdOptions(api_gateway, datasetId));
    return queryClient.ensureQueryData(getDistributionsByDatasetIdOptions(api_gateway, datasetId));
  },
});
