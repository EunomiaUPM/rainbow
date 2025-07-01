import {createFileRoute, Link} from "@tanstack/react-router";
import {useGetParticipants} from "shared/src/data/participant-queries.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table.tsx";
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
import {useContext, useEffect, useMemo, useState} from "react";
import {PubSubContext} from "shared/src/context/PubSubContext.tsx";
import {Button} from "shared/src/components/ui/button.tsx";
import {Input} from "shared/src/components/ui/input.tsx";
import {Badge} from "shared/src/components/ui/badge";
import Heading from "shared/src/components/ui/heading";

// Icons
import {ArrowRight, Plus} from "lucide-react";
import {useGetOidc, useGetProviderDid, useWalletOnboard} from "../../../../shared/src/data/wallet-mutations.ts";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage
} from "shared/src/components/ui/form.tsx";
import {SubmitHandler, useForm} from "react-hook-form";
import QRCode from "react-qr-code";


type Inputs = {
    providerBaseUrl: string
}

export const Route = createFileRoute("/participants/")({
    component: RouteComponent,
});

function RouteComponent() {
    const {data: participants} = useGetParticipants();
    const {lastHighLightedNotification} = useContext(PubSubContext)!;
    const {mutateAsync: onboardAsync} = useWalletOnboard()
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
    const {mutateAsync: didGetterAsync} = useGetProviderDid();
    const form = useForm<Inputs>({
        defaultValues: {
            providerBaseUrl: ""
        },
    });
    const {handleSubmit, control} = form;
    const [did, setDid] = useState<string | null>(null)
    const [providerBaseUrl, setProviderBaseUrl] = useState<string | null>(null)
    const [oidc, setOidc] = useState<string | null>(null)
    const {mutateAsync: oidcGetterAsync} = useGetOidc()
    const [copyStatus, setCopyStatus] = useState('');

    const onSubmitDidGetter: SubmitHandler<Inputs> = async data => {
        let did = await didGetterAsync({
            did_url: data.providerBaseUrl
        })
        setProviderBaseUrl(data.providerBaseUrl)
        setDid(did.id)
    }

    const hasProvider = useMemo(() => {
        const participant = participants.find(p => p.participant_type == "Provider")
        if (!participant) {
            return false
        } else {
            return true
        }
    }, participants)

    const onboardHandler = async () => {
        await onboardAsync({
            api_gateway
        })
    }
    const getOidcHandler = async () => {
        const oidc = await oidcGetterAsync({
            api_gateway,
            content: {
                url: providerBaseUrl + "/api/v1/access",
                id: did!,
                actions: "talk",
                slug: "provider"
            }
        })
        setOidc(oidc)
    }

    useEffect(() => {
        if (did) {
            getOidcHandler()
        }
    }, [did]);

    const handleCopy = async () => {
        try {
            await navigator.clipboard.writeText(oidc!);
            setCopyStatus('Copied!');
            setTimeout(() => setCopyStatus(''), 2000); // Clear message after 2 seconds
        } catch (err) {
            console.error('Failed to copy: ', err);
            setCopyStatus('Failed to copy.');
            setTimeout(() => setCopyStatus(''), 2000); // Clear message after 2 seconds
        }
    };
    const handleCancel = async () => {
        setDid(null)
        setDid(null)
        setProviderBaseUrl(null)
        setCopyStatus("")
        form.reset()
    };

    return (
        <div>
            <div className="pb-3 w-full flex justify-between items-center">
                <div className="basis-3/5">
                    <Input type="search"></Input>
                </div>

                {/* DRAWER ADD PARTICIPANT*/}
                <Drawer direction={"right"}>
                    <DrawerTrigger>
                        <Button>
                            Authenticate in provider
                            <Plus className="mb-1"/>
                        </Button>
                    </DrawerTrigger>
                    <DrawerContent>
                        <DrawerHeader>
                            <DrawerTitle>
                                <Heading level="h5" className="text-current">
                                    New Participant
                                </Heading>
                            </DrawerTitle>
                        </DrawerHeader>
                        <DrawerBody>

                            {/**/}
                            {/**/}
                            {/**/}
                            {/**/}
                            {!did && (
                                <Form {...form}>
                                    <form onSubmit={handleSubmit(onSubmitDidGetter)} className="space-y-6">
                                        {/* Consumer Participant Field */}
                                        <FormField
                                            control={control}
                                            name="providerBaseUrl"
                                            render={({field}) => ( // <--
                                                <FormItem>
                                                    <FormLabel>Provider base url:</FormLabel>
                                                    <div>
                                                        <FormControl>
                                                            <Input {...field} />
                                                        </FormControl>
                                                        <FormDescription className="text-sm text-gray-500 mt-1">Provide
                                                            base
                                                            url</FormDescription>
                                                        <FormMessage/>
                                                    </div>
                                                </FormItem>
                                            )}
                                        />
                                        <Button type="submit"
                                                className="w-full">
                                            Submit Request
                                        </Button>
                                    </form>
                                </Form>
                            )}

                            {/**/}
                            {/**/}
                            {/**/}
                            {/**/}
                            {(did && oidc) && (
                                <div>
                                    <div className="max-w-[400px]">
                                        {oidc != "" &&
                                            <QRCode size={32} value={oidc}
                                                    style={{height: "auto", maxWidth: "100%", width: "100%"}}

                                                    fgColor="#fff" bgColor="#0a0a1b"/>}
                                    </div>
                                    <div>
                                        <div className="break-words">{oidc}</div>
                                        <Button onClick={handleCopy}>{copyStatus || 'Copy Text'}</Button>
                                        <Button onClick={handleCancel}>Cancel</Button>
                                    </div>
                                </div>
                            )}


                        </DrawerBody>
                        <DrawerFooter>
                            <DrawerClose className="flex justify-start gap-4">
                                <Button variant="ghost" className="w-40">
                                    Cancel
                                </Button>
                                {/* <Button className="w-40">Add Participant</Button> */}
                            </DrawerClose>
                        </DrawerFooter>
                    </DrawerContent>
                </Drawer>
                {/* /DRAWER ADD PARTICIPANT*/}
            </div>


            <div>
                {!hasProvider && (<div>
                    mmm... parece que no hay un consumer..., tienes que hacer onboard en tu wallet
                </div>)}
                <Button onClick={() => onboardHandler()}>
                    Onboard!!!!
                </Button>
            </div>


            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>Participant ID</TableHead>
                        <TableHead>Identity Token</TableHead>
                        <TableHead>Participant Type</TableHead>
                        <TableHead>Base URL</TableHead>
                        {/* <TableHead>Extra Info</TableHead> */}
                        <TableHead>Link</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {participants.map((participant) => (
                        <TableRow
                            key={participant.participant_id.slice(0, 20)}
                            className={
                                participant.participant_id === lastHighLightedNotification
                                    ? "bg-blue-200"
                                    : ""
                            }
                        >
                            <TableCell>
                                <Badge variant={"info"}>
                                    {participant.participant_id.slice(9, 20) + "..."}
                                </Badge>
                            </TableCell>
                            <TableCell>{participant.token?.slice(0, 20) + "..."}</TableCell>
                            <TableCell>
                                <Badge variant={"role"} role={participant.participant_type}>
                                    {participant.participant_type}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                <Badge variant={"info"}>{participant.base_url}</Badge>
                            </TableCell>
                            {/* <TableCell>{JSON.stringify(participant.extra_fields)}</TableCell> */}
                            <TableCell>
                                <Link
                                    to="/participants/$participantId"
                                    params={{participantId: participant.participant_id}}
                                >
                                    <Button variant="link">
                                        See details
                                        <ArrowRight/>
                                    </Button>
                                </Link>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    );
}
