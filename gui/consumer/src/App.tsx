import dayjs from "dayjs";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table";
import { useFetchConsumerCallbacks } from "./hooks/useFetchConsumerCallbacks";
import { EyeIcon, PencilIcon } from "lucide-react";

const App = () => {
  const { data, isError, error } = useFetchConsumerCallbacks();

  if (isError) {
    return <div>{error!.message}</div>;
  }

  return (
    <div className="container">
      <h1>Dataspace consumer</h1>
      <Table>
        <TableCaption>Ongoing consumer callbacks</TableCaption>
        <TableHeader>
          <TableRow>
            <TableHead className="sticky left-0 top-0 bg-white border-r border-gray-400">Actions:</TableHead>
            <TableHead>Callback Id:</TableHead>
            <TableHead>Created at:</TableHead>
            <TableHead>Updated at:</TableHead>
            <TableHead>Provider pid:</TableHead>
            <TableHead>Consumer pid:</TableHead>
            <TableHead>Data Address:</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {data &&
            data.map((d: TransferCallbackModel, i: number) => (
              <TableRow key={i}>
                <TableCell className="sticky left-0 top-0 bg-white border-r border-gray-400">
                  <div className="flex gap-2 items-center">
                    <PencilIcon width={16} />
                    <EyeIcon width={16} />
                  </div>
                </TableCell>
                <TableCell className="truncate overflow-hidden w-1/12">
                  {d.id}
                </TableCell>
                <TableCell className="whitespace-nowrap">
                  {dayjs(d.created_at).format("DD/MM/YYYY HH:mm:ss.SSS")}
                </TableCell>
                <TableCell className="whitespace-nowrap">
                  {d.updated_at
                    ? dayjs(d.updated_at).format("DD/MM/YYYY HH:mm:ss.SSS")
                    : "-"}
                </TableCell>
                <TableCell className="truncate overflow-hidden w-1/12">
                  {d.provider_pid ? d.provider_pid : "-"}
                </TableCell>
                <TableCell className="truncate overflow-hidden w-1/12">
                  {d.consumer_pid ? d.consumer_pid : "-"}
                </TableCell>
                <TableCell className="truncate overflow-hidden w-1/12">
                  {d.data_address ? d.data_address["dspace:endpoint"] : "-"}
                </TableCell>
              </TableRow>
            ))}
        </TableBody>
      </Table>
    </div>
  );
};

export default App;
