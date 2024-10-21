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
                <TableCell>{d.id}</TableCell>
                <TableCell>
                  {dayjs(d.created_at).format("DD/MM/YYYY HH:mm:ss.SSS")}
                </TableCell>
                <TableCell>
                  {d.updated_at
                    ? dayjs(d.updated_at).format("DD/MM/YYYY HH:mm:ss.SSS")
                    : "-"}
                </TableCell>
                <TableCell>{d.provider_pid ? d.provider_pid : "-"}</TableCell>
                <TableCell>{d.consumer_pid ? d.consumer_pid : "-"}</TableCell>
                <TableCell>
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
