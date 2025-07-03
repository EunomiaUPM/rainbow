import { createFileRoute, Link } from "@tanstack/react-router";
import { useGetAgreements } from "shared/src/data/agreement-queries";
// import { useGetDatahubCatalogs } from "shared/src/data/datahub-catalog-queries";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import { Button } from "shared/src/components/ui/button.tsx";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { ArrowRight } from "lucide-react";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "shared/src/components/ui/dialog";

export const Route = createFileRoute("/agreements/")({
  component: RouteComponent,
});
function RouteComponent() {
  const { data: agreements } = useGetAgreements();
  agreements.map((agreement) => {
    console.log(agreement);
  });

  return (
    <div>
      <div>
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Agreement Id</TableHead>
              {/* <TableHead>Related Message</TableHead> */}
              <TableHead>
                {/*Consumer Participant Id */}
                Provider Participant Id
              </TableHead>

              <TableHead>Status</TableHead>
              <TableHead>Created at</TableHead>
              <TableHead>Actions</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {agreements.map((agreement) => (
              <TableRow key={agreement.agreement_id.slice(9, 20)}>
                <TableCell>
                  <Badge variant={"info"}>
                    {agreement.agreement_id.slice(9, 20) + "..."}
                  </Badge>
                </TableCell>
                <TableCell>
                  <Badge variant={"info"}>
                    {agreement.provider_participant_id.slice(9, 20) + "..."}
                  </Badge>
                </TableCell>
                {/* <TableCell>
                  <Badge variant={"info"}>
                    {agreement.cn_message_id?.slice(9, 20) + "..."}
                  </Badge>
                  </div> */}
                {/* </TableCell> */}
                <TableCell>
                  <Badge
                    variant={"status"}
                    state={agreement.active ? "ACTIVE" : "INACTIVE"}
                  >
                    {agreement.active ? "ACTIVE" : "INACTIVE"}
                  </Badge>
                </TableCell>
                <TableCell>
                  {dayjs(agreement.created_at).format("DD/MM/YY HH:mm")}
                </TableCell>
                <TableCell>
                  <Dialog>
                    <DialogTrigger asChild>
                      <Button variant="outline" size="sm">
                        REQUEST TRANSFEREsNCE
                        {/* <ArrowRight /> */}
                      </Button>
                    </DialogTrigger>
                    <DialogContent>
                      <DialogHeader>
                        <DialogTitle>Transfer request dialog</DialogTitle>
                        <DialogDescription className="max-w-full flex flex-wrap break-all">
                          <span className="max-w-full flex flex-wrap">
                            Select the transference's parameters for 
                         <Badge variant="info">  {agreement.agreement_id.slice(9, 30) + "[...]"} </Badge> 
                            associated to dataset $xxxxx
                          </span>
                        </DialogDescription>

                      </DialogHeader>
                    </DialogContent>
                  </Dialog>
                </TableCell>
                <TableCell>
                  <Link
                    to="/agreements/$agreementId"
                    params={{ agreementId: agreement.agreement_id }}
                  >
                    <Button variant="link">
                      See details
                      <ArrowRight />
                    </Button>
                  </Link>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
