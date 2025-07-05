import { createFileRoute } from "@tanstack/react-router";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { SubmitHandler } from "react-hook-form";
import { useGetDatahubDataset } from "../../../../../shared/src/data/datahub-catalog-queries.ts";
import { useGetPoliciesByDatasetId } from "shared/src/data/policy-queries.ts";
import { usePostNewPolicyInDataset } from "shared/src/data/catalog-mutations.ts";
import { useContext } from "react";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "shared/src/context/GlobalInfoContext.tsx";
import { PolicyWrapperNew } from "shared/src/components/PolicyWrapperNew.tsx";
import { PolicyWrapperShow } from "shared/src/components/PolicyWrapperShow.tsx";
import Heading from "shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list";
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
import {  Plus } from "lucide-react";

type Inputs = {
  odrl: string;
};

function RouteComponent() {
  const { datasetId } = Route.useParams();
  const { data: dataset } = useGetDatahubDataset(datasetId);
  const { data: policies } = useGetPoliciesByDatasetId(datasetId);
  const { mutateAsync: createPolicyAsync, isPending } =
    usePostNewPolicyInDataset();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(
    GlobalInfoContext
  )!;

  const onSubmit: SubmitHandler<Inputs> = (data) => {
    // @ts-ignore
    createPolicyAsync({
      api_gateway,
      datasetId,
      content: {
        offer: JSON.stringify(data),
      },
    });
  };

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
          {" "}
          ODRL Policies{" "}
        </Heading>
        <Drawer direction={"right"}>
          <DrawerTrigger>
            <Button variant="default" size="sm" className="mb-1 ml-3">
              Add ODRL policy
              <Plus className="" />
            </Button>
          </DrawerTrigger>
          <DrawerContent>
            <DrawerHeader className="px-8">
              <DrawerTitle>
                <Heading level="h4" className="text-current mb-1 ">
                  New ODRL Policy
                </Heading>
                <p className="font-normal text-brand-sky">
                  {" "}
                  for Dataset
                  <Badge variant="info" size="sm" className="ml-2">
                    {" "}
                    {dataset.name}
                  </Badge>
                </p>
              </DrawerTitle>
            </DrawerHeader>
            <PolicyWrapperNew onSubmit={onSubmit} />
          </DrawerContent>
        </Drawer>
      </div>

        <div className="grid grid-cols-2 gap-4">
          {policies.map((policy) => (
            <PolicyWrapperShow policy={policy} />
          ))}
        </div>

    
    </div>
  );
}

export const Route = createFileRoute(
  "/datahub-catalog/$catalogId/dataset/$datasetId"
)({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
