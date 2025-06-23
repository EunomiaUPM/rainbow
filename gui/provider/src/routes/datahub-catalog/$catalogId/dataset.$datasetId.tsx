import {createFileRoute} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {SubmitHandler, useForm} from "react-hook-form";
import {useGetDatahubDataset} from "../../../../../shared/src/data/datahub-catalog-queries.ts";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage
} from "shared/src/components/ui/form.tsx";
import {Textarea} from "shared/src/components/ui/textarea.tsx";
import {Button} from "shared/src/components/ui/button.tsx";
import {useGetPoliciesByDatasetId} from "shared/src/data/policy-queries.ts";
import {usePostNewPolicyInDataset} from "shared/src/data/catalog-mutations.ts";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";


type Inputs = {
    odrl: string
}

function RouteComponent() {
    const {datasetId} = Route.useParams()
    const {data: dataset} = useGetDatahubDataset(datasetId)
    const {data: policies} = useGetPoliciesByDatasetId(datasetId)
    const {mutateAsync: createPolicyAsync, isPending} = usePostNewPolicyInDataset()
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!
    const form = useForm<Inputs>({
        defaultValues: {
            odrl: "{\"permission\":[{\"action\":\"use\",\"constraint\":[{\"rightOperand\":\"user\",\"leftOperand\":\"did:web:hola.es\",\"operator\":\"eq\"}]}],\"obligation\":[],\"prohibition\":[]}",
        },
    })
    const onSubmit: SubmitHandler<Inputs> = data => {
        // @ts-ignore
        createPolicyAsync({
            api_gateway,
            datasetId,
            content: {
                offer: data.odrl
            }
        })
        form.reset()
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
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Policy Id</TableHead>
                        <TableHead>Policy Target</TableHead>
                        <TableHead>ODRL Content</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {policies.map((policy) => (
                        <TableRow key={policy["@id"].slice(0, 20)}>
                            <TableCell>
                                {policy["@id"].slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {policy.target?.slice(0, 20) + "..."}
                            </TableCell>
                            <TableCell>
                                {JSON.stringify(policy)}
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
        <div>
            <h2>Create new odrl policy</h2>
            <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit)}>
                    <FormField
                        disabled={isPending}
                        control={form.control}
                        name="odrl"
                        render={({field}) => (
                            <FormItem>
                                <FormLabel>Odrl</FormLabel>
                                <FormControl>
                                    <Textarea {...field} />
                                </FormControl>
                                <FormDescription>Provide the ODRL policy content</FormDescription>
                                <FormMessage/>
                            </FormItem>
                        )}
                    />
                    <Button type="submit">Enviar {isPending && <span>- loading...</span>}</Button>
                </form>
            </Form>
        </div>

    </div>

}

export const Route = createFileRoute('/datahub-catalog/$catalogId/dataset/$datasetId')({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,

})