import {createFileRoute} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {FormProvider, useForm} from "react-hook-form"; // Import FormProvider
import {useContext, useEffect} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import {
    useBusinessGetPoliciesByDatasetId,
    useGetBusinessDatahubDataset,
    useGetBusinessPolicyTemplates
} from "shared/src/data/business-queries.ts";
import {AuthContext, AuthContextType} from "shared/src/context/AuthContext.tsx";
import {
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "shared/src/components/ui/form.tsx";
import {Textarea} from "shared/src/components/ui/textarea.tsx";
import {Button} from "shared/src/components/ui/button.tsx";
import {Input} from "shared/src/components/ui/input.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "shared/src/components/ui/select.tsx";
import {usePostBusinessNewPolicyInDataset} from "shared/src/data/business-mutations.ts";
import {Dialog, DialogTrigger} from "shared/src/components/ui/dialog.tsx";
import {BusinessRemovePolicyDialog} from "shared/src/components/BusinessRemovePolicyDialog.tsx";
import {BusinessRequestAccessDialog} from "shared/src/components/BusinessRequestAccessDialog.tsx";


type DynamicFormValues = {
    [key: string]: string;
};

function RouteComponent() {
    const {catalogId, datasetId} = Route.useParams()
    const {participant} = useContext<AuthContextType | null>(AuthContext)!;
    const {data: dataset} = useGetBusinessDatahubDataset(datasetId)
    const {data: policies} = useBusinessGetPoliciesByDatasetId(catalogId, datasetId)
    const {data: policy_templates} = useGetBusinessPolicyTemplates() as { data: PolicyTemplate[] };
    const {mutateAsync: createPolicyAsync, isPending} = usePostBusinessNewPolicyInDataset()
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!
    const form = useForm<DynamicFormValues>({
        defaultValues: {},
    })

    useEffect(() => {
        if (policy_templates) {
            const initialValues: DynamicFormValues = {};
            policy_templates.forEach(template => {
                Object.entries(template.operand_options).forEach(([key, value]) => {
                    if (key && value.defaultValue !== undefined) {
                        initialValues[key] = value.defaultValue;
                    }
                });
            });
            form.reset(initialValues);
        }
    }, [policy_templates, form]);

    const onSubmit = async (formData: DynamicFormValues, currentPolicyTemplate: PolicyTemplate) => {
        let odrlContent = JSON.parse(JSON.stringify(currentPolicyTemplate.content));

        for (const key in formData) {
            if (formData.hasOwnProperty(key)) {
                const value = formData[key];
                let contentString = JSON.stringify(odrlContent);
                const escapedKey = key.replace(/[$]/g, '\\$&');
                contentString = contentString.replace(new RegExp(escapedKey, 'g'), value);
                odrlContent = JSON.parse(contentString);
            }
        }

        console.log("ODRL conformed:", odrlContent);
        await createPolicyAsync({
            api_gateway,
            datasetId,
            catalogId,
            content: {
                offer: odrlContent
            }
        });
        form.reset();
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
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Policy Id</TableHead>
                            <TableHead>Policy Target</TableHead>
                            <TableHead>ODRL Content</TableHead>
                            <TableHead>Actions</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {/* Use optional chaining for policies */}
                        {policies?.map((policy) => (
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

                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>

            {/* Render this section only if participant is a Provider and policy_templates are loaded */}
            {
                participant?.participant_type == "Provider" && policy_templates &&
                (<>
                    <div>
                        <h2>Create new ODRL policy from template</h2>
                        <Table className="text-sm">
                            <TableHeader>
                                <TableRow>
                                    <TableHead>Policy Template Id</TableHead>
                                    <TableHead>Policy Template Title</TableHead>
                                    <TableHead>Policy Template Description</TableHead>
                                    <TableHead>Policy Template Content</TableHead>
                                    <TableHead>Policy Template Options</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                {/* Map through each policy template to display its information and a form */}
                                {policy_templates.map(policy_template => (
                                    <TableRow key={policy_template.id}> {/* Add a unique key for each row */}
                                        <TableCell>
                                            {policy_template.id.slice(0, 20) + "..."}
                                        </TableCell>
                                        <TableCell>
                                            {policy_template.title}
                                        </TableCell>
                                        <TableCell>
                                            {policy_template.description}
                                        </TableCell>
                                        <TableCell>
                                            {JSON.stringify(policy_template.content)}
                                        </TableCell>
                                        <TableCell>
                                            {/* Use FormProvider explicitly to ensure context is available for FormField */}
                                            <FormProvider {...form}>
                                                {/* The actual HTML form element */}
                                                <form
                                                    onSubmit={form.handleSubmit((data) => onSubmit(data, policy_template))}
                                                    className="space-y-4" // Add some spacing to the form
                                                >
                                                    {/* Map through each operand option to render dynamic form fields */}
                                                    {Object.entries(policy_template.operand_options).map(([optionKey, optionValue]) => {
                                                        return (
                                                            <FormField
                                                                key={optionKey} // Unique key for each FormField
                                                                disabled={isPending}
                                                                control={form.control}
                                                                name={optionKey} // The name of the field in react-hook-form
                                                                render={({field}) => {
                                                                    let inputComponent; // Declare a variable to hold the input component inside render

                                                                    // Conditionally assign the correct input component
                                                                    if (optionValue.formType === "text") {
                                                                        inputComponent =
                                                                            <Input {...field} type="text"/>;
                                                                    } else if (optionValue.formType === "datetime") {
                                                                        inputComponent =
                                                                            <Input {...field} type="datetime-local"/>;
                                                                    } else if (optionValue.formType === "select") {
                                                                        inputComponent = (
                                                                            <Select onValueChange={field.onChange}
                                                                                    defaultValue={field.value}>
                                                                                <SelectTrigger>
                                                                                    <SelectValue
                                                                                        placeholder="Select an option"/>
                                                                                </SelectTrigger>
                                                                                <SelectContent>
                                                                                    {optionValue.options?.map(opt => (
                                                                                        <SelectItem key={opt.value}
                                                                                                    value={opt.value}>
                                                                                            {opt.label.find(l => l["@language"] == "en")?.["@value"]}
                                                                                        </SelectItem>
                                                                                    ))}
                                                                                </SelectContent>
                                                                            </Select>
                                                                        );
                                                                    } else {
                                                                        // Default to Textarea if no specific formType or for generic text
                                                                        inputComponent = <Textarea {...field} />;
                                                                    }

                                                                    return (
                                                                        <FormItem>
                                                                            <FormLabel>Option: {optionKey}</FormLabel>
                                                                            <FormControl>
                                                                                {/* Render the single input component */}
                                                                                {inputComponent}
                                                                            </FormControl>
                                                                            <FormDescription>
                                                                                {optionValue.label.find(l => l["@language"] == "en")?.["@value"]}
                                                                            </FormDescription>
                                                                            <FormMessage/>
                                                                        </FormItem>
                                                                    );
                                                                }}
                                                            />
                                                        );
                                                    })}
                                                    <Button type="submit" disabled={isPending}>
                                                        Enviar {isPending && <span>- loading...</span>}
                                                    </Button>
                                                </form>
                                            </FormProvider>
                                        </TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
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
