import React, {useEffect} from "react"
import {List, ListItem, ListItemKey} from "shared/src/components/ui/list";
import Heading from "shared/src/components/ui/heading";
import {Badge} from "shared/src/components/ui/badge";
import {Accordion, AccordionContent, AccordionItem, AccordionTrigger} from "shared/src/components/ui/accordion";
import {Input} from "shared/src/components/ui/input";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "shared/src/components/ui/select";
import {FormProvider, useForm} from "react-hook-form";
import {FormField} from "shared/src/components/ui/form";
import {Button} from "shared/src/components/ui/button";


type DynamicFormValues = {
    [key: string]: string;
};


export const PolicyTemplateWrapperEdit = ({policyTemplate, onSubmit}: {
    policyTemplate: PolicyTemplate,
    onSubmit: (odrlContent: OdrlInfo) => Promise<void>
}) => {
    const form = useForm<DynamicFormValues>({
        defaultValues: {},
    })

    // Parse template into form data
    useEffect(() => {
        const initialValues: DynamicFormValues = {};
        Object.entries(policyTemplate.operand_options).forEach(([key, value]) => {
            if (key && value.defaultValue !== undefined) {
                initialValues[key] = value.defaultValue;
            }
        });
        form.reset(initialValues);
    }, [policyTemplate, form]);

    // On submit conform ODRL and use onSubmit method from parent component
    const submitHandler = async (formData: DynamicFormValues, currentPolicyTemplate: PolicyTemplate) => {
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
        await onSubmit(odrlContent)
        form.reset();
    }

    // Create fields
    const renderOperandOptions = (field, operand: string) => {
        const options = policyTemplate.operand_options[operand]
        const formType = options.formType
        const defaultValue = options.defaultValue
        const innerOptions = options.options
        let component;
        switch (formType) {
            case "date":
            case "text":
                component = <Input type={formType} defaultValue={defaultValue} {...field} />
                break;
            case "select":
                component = (<Select
                    defaultValue={defaultValue}
                    onValueChange={field.onChange}>
                    <SelectTrigger>
                        <SelectValue
                            placeholder="Select an option"/>
                    </SelectTrigger>
                    <SelectContent>
                        {innerOptions?.map(opt => (
                            <SelectItem key={opt.value}
                                        value={opt.value}>
                                {opt.label.find(l => l["@language"] == "en")?.["@value"]}
                            </SelectItem>
                        ))}
                    </SelectContent>
                </Select>)
                break;
            default:

        }
        return component
    }

    // Render form fields
    const renderOperandOptionsFormControl = (operand: string) => {
        const formComponent = <FormField render={({field}) => renderOperandOptions(field, operand)} name={operand}
                                         key={operand} control={form.control}/>
        return formComponent
    }

    return (<div>
        <FormProvider {...form}>
            <form
                onSubmit={form.handleSubmit((data) => submitHandler(data, policyTemplate))}
                className="space-y-4" // Add some spacing to the form
            >
                <List className="border border-white/30 bg-white/10 px-4 py-2 rounded-md justify-start">
                    <div className="flex">
                        <Heading level="h5" className="flex gap-3">
                            <div>Policy template with ID:</div>
                            <Badge variant="info" className="h-6">
                                {policyTemplate.id.slice(9, 29) + "[...]"}
                            </Badge>
                        </Heading>
                    </div>
                    <ListItem>
                        <ListItemKey>Title</ListItemKey>
                        <p>{policyTemplate.title}</p>
                    </ListItem>
                    <ListItem>
                        <ListItemKey>Description</ListItemKey>
                        <p>{policyTemplate.description}</p>
                    </ListItem>
                    <div className="h-5"></div>
                    <Heading level="h6"> ODRL CONTENT</Heading>
                    <div className="flex flex-col gap-2">
                        <div className="flex flex-col gap-4">
                            <Accordion type="single" collapsible className="w-full">
                                <AccordionItem
                                    value="item-1"
                                    className="bg-success-500/10 border border-success-600/20"
                                >
                                    <AccordionTrigger
                                        className="text-white/70 flex bg-success-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                                        <div className="flex items-center w-full">
                                            <p className="text-current">permission</p>
                                        </div>
                                    </AccordionTrigger>
                                    <AccordionContent className="relative">
                                        {
                                            (policyTemplate.content.permission == undefined || policyTemplate.content.permission?.length == 0) &&
                                            <p>No policy defined</p>
                                        }
                                        {
                                            policyTemplate.content.permission !== undefined && (
                                                (policyTemplate.content.permission || []).map((permission, i) => (<div>
                                                    <div className="policy-item-create">
                                                        {permission.action.toUpperCase()}
                                                        <div className="h-6"></div>
                                                        <p className="mb-2"> Constraints: </p>
                                                        {permission.constraint.map((constraint, j) => (
                                                            <div className="flex flex-col gap-2">
                                                                <div className="constraint-create flex gap-3">
                                                                    <div className="flex flex-col">
                                                                        <p className="text-xs text-gray-400 mb-1">
                                                                            Left Operand:
                                                                        </p>
                                                                        <p>{constraint.leftOperand}</p>
                                                                    </div>
                                                                    <div className="flex flex-col">
                                                                        <p className="text-xs text-gray-400 mb-1">
                                                                            Operator:
                                                                        </p>
                                                                        <p>{constraint.operator}</p>
                                                                    </div>
                                                                    <div className="flex flex-col">
                                                                        <p className="text-xs text-gray-400 mb-1">
                                                                            Right Operand:
                                                                        </p>
                                                                        <div>
                                                                            {renderOperandOptionsFormControl(constraint.rightOperand)}
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        ))}
                                                    </div>
                                                </div>))
                                            )
                                        }
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                            <Accordion type="single" collapsible className="w-full">
                                <AccordionItem
                                    value="item-1"
                                    className="bg-warn-500/10 border border-warn-600/20"
                                >
                                    <AccordionTrigger
                                        className="text-white/70 flex bg-warn-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                                        <div className="flex items-center w-full">
                                            <p className="text-current">obligation</p>
                                        </div>
                                    </AccordionTrigger>
                                    <AccordionContent className="relative">
                                        {
                                            (policyTemplate.content.obligation == undefined || policyTemplate.content.obligation?.length == 0) &&
                                            <p>No policy defined</p>
                                        }
                                        {(policyTemplate.content.obligation || []).map((obligation, i) => (<div>
                                            <div className="policy-item-create">
                                                {obligation.action.toUpperCase()}
                                                <div className="h-6"></div>
                                                <p className="mb-2"> Constraints: </p>
                                                {obligation.constraint.map((constraint, j) => (
                                                    <div className="flex flex-col gap-2">
                                                        <div className="constraint-create flex gap-3">
                                                            <div className="flex flex-col">
                                                                <p className="text-xs text-gray-400 mb-1">
                                                                    Left Operand:
                                                                </p>
                                                                <p>{constraint.leftOperand}</p>
                                                            </div>
                                                            <div className="flex flex-col">
                                                                <p className="text-xs text-gray-400 mb-1">
                                                                    Operator:
                                                                </p>
                                                                <p>{constraint.operator}</p>
                                                            </div>
                                                            <div className="flex flex-col">
                                                                <p className="text-xs text-gray-400 mb-1">
                                                                    Right Operand:
                                                                </p>
                                                                <div>
                                                                    {renderOperandOptionsFormControl(constraint.rightOperand)}
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                ))}
                                            </div>
                                        </div>))}
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                            <Accordion type="single" collapsible className="w-full">
                                <AccordionItem
                                    value="item-1"
                                    className="bg-danger-500/10 border border-danger-600/20"
                                >
                                    <AccordionTrigger
                                        className="text-white/70 flex bg-danger-500/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                                        <div className="flex items-center w-full">
                                            <p className="text-current">prohibition</p>
                                        </div>
                                    </AccordionTrigger>
                                    <AccordionContent className="relative">
                                        {
                                            (policyTemplate.content.prohibition == undefined || policyTemplate.content.prohibition?.length == 0) &&
                                            <p>No policy defined</p>
                                        }
                                        {(policyTemplate.content.prohibition || []).map((prohibition, i) => (<div>
                                            <div className="policy-item-create">
                                                {prohibition.action.toUpperCase()}
                                                <div className="h-6"></div>
                                                <p className="mb-2"> Constraints: </p>
                                                {prohibition.constraint.map((constraint, j) => (
                                                    <div className="flex flex-col gap-2">
                                                        <div className="constraint-create flex gap-3">
                                                            <div className="flex flex-col">
                                                                <p className="text-xs text-gray-400 mb-1">
                                                                    Left Operand:
                                                                </p>
                                                                <p>{constraint.leftOperand}</p>
                                                            </div>
                                                            <div className="flex flex-col">
                                                                <p className="text-xs text-gray-400 mb-1">
                                                                    Operator:
                                                                </p>
                                                                <p>{constraint.operator}</p>
                                                            </div>
                                                            <div className="flex flex-col">
                                                                <p className="text-xs text-gray-400 mb-1">
                                                                    Right Operand:
                                                                </p>
                                                                <div>
                                                                    {renderOperandOptionsFormControl(constraint.rightOperand)}
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                ))}
                                            </div>
                                        </div>))}
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                        </div>
                    </div>
                </List>
                <Button type="submit">
                    Enviar
                </Button>
            </form>
        </FormProvider>
    </div>)
}