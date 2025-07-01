import {createFileRoute} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {SubmitHandler} from "react-hook-form";
import {useGetDatahubDataset} from "../../../../../shared/src/data/datahub-catalog-queries.ts";
import {useGetPoliciesByDatasetId} from "shared/src/data/policy-queries.ts";
import {usePostNewPolicyInDataset} from "shared/src/data/catalog-mutations.ts";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import {PolicyWrapperNew} from "shared/src/components/PolicyWrapperNew.tsx";
import {PolicyWrapperShow} from "shared/src/components/PolicyWrapperShow.tsx";


type Inputs = {
    odrl: string
}

function RouteComponent() {
    const {datasetId} = Route.useParams()
    const {data: dataset} = useGetDatahubDataset(datasetId)
    const {data: policies} = useGetPoliciesByDatasetId(datasetId)
    const {mutateAsync: createPolicyAsync, isPending} = usePostNewPolicyInDataset()
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!

    const onSubmit: SubmitHandler<Inputs> = data => {
        // @ts-ignore
        createPolicyAsync({
            api_gateway,
            datasetId,
            content: {
                offer: JSON.stringify(data)
            }
        })
    }


    return <div className="space-y-4">
        <h2>Dataset info with id: {dataset.name} </h2>
        <div>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Key</TableHead>
                        <TableHead>Value</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {dataset.custom_properties.map((property => (
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
                {policies.map((policy) => (
                    <PolicyWrapperShow policy={policy}/>
                ))}
            </div>

        </div>
        <div>
            <h2>Create new odrl policy</h2>
            <div>
                <PolicyWrapperNew onSubmit={onSubmit}/>
            </div>
        </div>

    </div>

}

export const Route = createFileRoute('/datahub-catalog/$catalogId/dataset/$datasetId')({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,

})