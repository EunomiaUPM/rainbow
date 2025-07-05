import {createFileRoute} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import {
    useBusinessGetPoliciesByDatasetId,
    useGetBusinessDatahubDataset,
    useGetBusinessPolicyTemplates
} from "shared/src/data/business-queries.ts";
import {AuthContext, AuthContextType} from "shared/src/context/AuthContext.tsx";
import {Button} from "shared/src/components/ui/button.tsx";
import {usePostBusinessNewPolicyInDataset} from "shared/src/data/business-mutations.ts";
import {BusinessRemovePolicyDialog} from "shared/src/components/BusinessRemovePolicyDialog.tsx";
import {BusinessRequestAccessDialog} from "shared/src/components/BusinessRequestAccessDialog.tsx";
import {PolicyWrapperShow} from "shared/src/components/PolicyWrapperShow.tsx";
import {PolicyTemplateWrapperEdit} from "../../../../../shared/src/components/PolicyTemplateWrapperEdit.tsx";
import Heading from "shared/src/components/ui/heading.tsx";
import {  Plus } from "lucide-react";
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
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list";
import { Badge } from "shared/src/components/ui/badge";
import { PolicyWrapperNew } from "shared/src/components/PolicyWrapperNew";


function RouteComponent() {
    const {catalogId, datasetId} = Route.useParams()
    const {participant} = useContext<AuthContextType | null>(AuthContext)!;
    const {data: dataset} = useGetBusinessDatahubDataset(datasetId)
    const {data: policies} = useBusinessGetPoliciesByDatasetId(catalogId, datasetId)
    const {data: policy_templates} = useGetBusinessPolicyTemplates() as { data: PolicyTemplate[] };
    const {mutateAsync: createPolicyAsync} = usePostBusinessNewPolicyInDataset()
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!

    const onSubmit = async (odrlContent: OdrlInfo) => {
        await createPolicyAsync({
            api_gateway,
            datasetId,
            catalogId,
            content: {
                offer: odrlContent
            }
        });
    }

    return (
        <div className="space-y-4">
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
             {participant?.participant_type == "Provider" && policy_templates &&
            <Button variant="default" size="sm" className="mb-1 ml-3">
              Add ODRL policy
              <Plus className="" />
            </Button>}
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
          {policies && policies.map((policy) => (
            <PolicyWrapperShow participant={participant} policy={policy} catalogId={catalogId} datasetId={datasetId}>
            </PolicyWrapperShow> 
          ))}
        </div>
        

    
      
        </div>
    );
}

export const Route = createFileRoute('/datahub-catalog/$catalogId/dataset/$datasetId')({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
})
