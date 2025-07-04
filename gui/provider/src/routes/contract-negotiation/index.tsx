import {createFileRoute, Link} from "@tanstack/react-router";
import {useMemo, useState} from "react";
import dayjs from "dayjs";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import {Button} from "shared/src/components/ui/button.tsx";
import {Badge} from "shared/src/components/ui/badge.tsx";
import {Input} from "shared/src/components/ui/input.tsx";
import {useGetContractNegotiationProcesses} from "shared/src/data/contract-queries.ts";
import {ContractNegotiationActions} from "shared/src/components/ContractNegotiationActions";
import {ArrowRight} from "lucide-react";
import {RouteComponent as CounterOfferForm} from "@/routes/contract-negotiation/counter-offer";


const RouteComponent = () => {
    const {data: cnProcesses} = useGetContractNegotiationProcesses();
    const cnProcessesSorted = useMemo(() => {
        if (!cnProcesses) return [];
        return [...cnProcesses].sort((a, b) => {
            return (
                new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
            );
        });
    }, [cnProcesses]);
    const [openCounterOffer, setOpenCounterOffer] = useState(false)
    const [selectedCnProcess, setSelectedCnProcess] = useState("")
    return (
        <div>
            <div className={openCounterOffer === true ? `flex fixed   z-50` : `hidden absolute`}>
                <CounterOfferForm process={selectedCnProcess}/>
            </div>
            <div className=" pb-3 flex justify-between items-start">
                <div className=" basis-3/5">
                    <Input type="search"></Input>
                </div>
            </div>
            <Table className="text-sm">
                <TableHeader>
                    <TableRow>
                        <TableHead>ProviderPid</TableHead>
                        <TableHead>ConsumerPid</TableHead>
                        <TableHead>State</TableHead>
                        <TableHead>Client Type</TableHead>
                        <TableHead>Created at</TableHead>
                        <TableHead>Actions</TableHead>
                        <TableHead>Link</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {cnProcessesSorted.map((cnProcess) => (
                        <TableRow key={cnProcess.provider_id?.slice(0, 20)}>
                            <TableCell>
                                <Badge variant={"info"}>
                                    {cnProcess.provider_id?.slice(9, 20) + "..."}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                <Badge variant={"info"}>
                                    {cnProcess.consumer_id?.slice(9, 20) + "..."}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                <Badge variant={"status"} state={cnProcess.state}>
                                    {cnProcess.state.replace("dspace:", "")}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                <Badge variant={"status"} state={cnProcess.state}>
                                    {cnProcess.is_business ? "Business" : "Standard"}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                {dayjs(cnProcess.created_at).format("DD/MM/YY - HH:mm")}
                            </TableCell>
                            <TableCell>
                                {console.log(selectedCnProcess, " selected cnproc")}
                                {cnProcess.state === "REQUESTED" ?
                                    <Button variant="outline" size="sm" className="" onClick={() =>
                                        // toggle dialog
                                    {
                                        (openCounterOffer === true ? setOpenCounterOffer(false) : setOpenCounterOffer(true));
                                        (setSelectedCnProcess(cnProcess))
                                    }


                                    }>
                                        C/Offer
                                    </Button> : ""}


                                <ContractNegotiationActions process={cnProcess} tiny={true}/>
                            </TableCell>
                            <TableCell>
                                <Link
                                    to="/contract-negotiation/$cnProcess"
                                    params={{cnProcess: cnProcess.provider_id}}
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
};

export const Route = createFileRoute("/contract-negotiation/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
});
