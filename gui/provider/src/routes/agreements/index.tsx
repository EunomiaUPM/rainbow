import { createFileRoute, Link } from "@tanstack/react-router";
import { useGetAgreements } from "shared/src/data/agreement-queries";
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

export const Route = createFileRoute("/agreements/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: agreements } = useGetAgreements();
  agreements.map((agreement) => {
    console.log(agreement.active);
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
              <TableHead>Related Message</TableHead>
              <TableHead>
                {/* <div className="flex gap-3">
                  <p>Consumer Participant Id</p>
                  <p> Provider Participant Id</p>
              </div> */}
              </TableHead>

              <TableHead>Status</TableHead>
              <TableHead>CreatedAt</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {agreements.map((agreement) => (
              <TableRow key={agreement.agreement_id.slice(9, 29)}>
                <TableCell>
                  <Badge variant={"info"}>
                    {agreement.agreement_id.slice(9, 29) + "[...]"}
                  </Badge>
                </TableCell>
                <TableCell>
                  {agreement.cn_message_id?.slice(9, 29) + "[...]"}
                </TableCell>
                <TableCell>
                  {/* <div className="flex flex-col gap-1">
                  <Badge variant={"info"}>
                    {agreement.consumer_participant_id?.slice(0, 20) + "..."}
                  </Badge>
              
                  <Badge variant={"info"}>
                    {agreement.provider_participant_id?.slice(0, 20) + "..."}
                  </Badge>
                  </div> */}
                </TableCell>
                <TableCell>
                  <Badge
                    variant={"status"}
                    state={agreement.active ? "ACTIVE" : "SUSPENDED"} // lo dejo para que ponga un color gris, pero se puede poner rojo
                  >
                    {agreement.active ? "ACTIVE" : "INACTIVE"}
                  </Badge>
                </TableCell>
                <TableCell>
                  {dayjs(agreement.created_at).format("DD/MM/YY HH:mm")}
                </TableCell>
                <TableCell>
                  <Link
                    to="/agreements/$agreementId"
                    params={{ agreementId: agreement.agreement_id }}
                  >
                    <Button variant="link">
                      See agreement
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
