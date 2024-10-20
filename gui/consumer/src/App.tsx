import { useQuery } from "@tanstack/react-query";
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

const App = () => {
  const { data, isError, error } = useQuery({
    queryKey: [],
    queryFn: async () => {
      const consumerCallbacks = await fetch(
        "http://localhost:1235/api/v1/callbacks"
      );
      return await consumerCallbacks.json();
    },
    refetchInterval: 10000,
    refetchIntervalInBackground: true,
  });

  if (isError) {
    return <div>{error.message}</div>;
  }

  return (
    <div className="">
      <h1>Dataspace consumer</h1>
      <Table>
        <TableCaption>Ongoing consumer callbacks</TableCaption>
        <TableHeader>
          <TableRow>
            <TableHead>Callback Id:</TableHead>
            <TableHead>Created at:</TableHead>
            <TableHead>Updated at:</TableHead>
            <TableHead>Data Address:</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {data &&
            data.map((d, i: number) => (
              <TableRow key={i}>
                <TableCell className="font-medium">{d.id}</TableCell>
                <TableCell>
                  {dayjs(d.created_at).format("DD/MM/YYYY HH:mm:ss.SSS")}
                </TableCell>
                <TableCell>
                  {d.updated_at
                    ? dayjs(d.updated_at).format("DD/MM/YYYY HH:mm:ss.SSS")
                    : "-"}
                </TableCell>
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
