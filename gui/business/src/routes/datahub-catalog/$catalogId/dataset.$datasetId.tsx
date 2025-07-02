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
import {Dialog, DialogTrigger} from "shared/src/components/ui/dialog.tsx";
import {BusinessRemovePolicyDialog} from "shared/src/components/BusinessRemovePolicyDialog.tsx";
import {BusinessRequestAccessDialog} from "shared/src/components/BusinessRequestAccessDialog.tsx";
import {PolicyWrapperShow} from "shared/src/components/PolicyWrapperShow.tsx";
import {PolicyTemplateWrapperEdit} from "../../../../../shared/src/components/PolicyTemplateWrapperEdit.tsx";


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
            <h2>Dataset info with id: {dataset?.name} </h2> {/* Use optional chaining */}
            <div>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Key</TableHead>
                            <TableHead>Value</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {/* Use optional chaining for dataset and custom_properties */}
                        {dataset?.custom_properties?.map((property => (
                            <TableRow key={property[0]}>
                                <TableCell>{property[0]}</TableCell>
                                <TableCell>{property[1]}</TableCell>
                            </TableRow>
                        )))}
                    </TableBody>
                </Table>
            </div>

            <div>
                <h2>ODRL Policies</h2>
                <div className="grid grid-cols-2 gap-4">
                    {policies.map(policy => (
                        <div>
                            <PolicyWrapperShow policy={policy}/>
                            {participant?.participant_type == "Provider" && <>
                                <TableCell>
                                    <Dialog>
                                        <DialogTrigger asChild>
                                            <Button variant="destructive" size="sm">Remove policy</Button>
                                        </DialogTrigger>
                                        <BusinessRemovePolicyDialog policy={policy} catalogId={catalogId}
                                                                    datasetId={datasetId}/>
                                    </Dialog>
                                </TableCell>
                            </>}
                            {participant?.participant_type == "Consumer" && <>
                                <TableCell>
                                    <Dialog>
                                        <DialogTrigger asChild>
                                            <Button variant="default" size="sm">Request access</Button>
                                        </DialogTrigger>
                                        <BusinessRequestAccessDialog policy={policy} catalogId={catalogId}
                                                                     datasetId={datasetId}/>
                                    </Dialog>
                                </TableCell>
                            </>}
                        </div>
                    ))}
                </div>
            </div>

            {/* Render this section only if participant is a Provider and policy_templates are loaded */}
            {
                participant?.participant_type == "Provider" && policy_templates &&
                (<>
                    <h2>Create new ODRL policy from template</h2>
                    <div className="grid grid-cols-2 gap-4">
                        {policy_templates.map(policy_template => (
                            <PolicyTemplateWrapperEdit policyTemplate={policy_template} onSubmit={onSubmit}/>))}
                    </div>
                </>)
            }
        </div>
    );
}

export const Route = createFileRoute('/datahub-catalog/$catalogId/dataset/$datasetId')({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
})
