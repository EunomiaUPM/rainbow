import React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "./ui/table";

export interface Column<T> {
  header: string;
  accessorKey?: keyof T;
  cell?: (item: T) => React.ReactNode;
  className?: string;
}

interface DataTableProps<T> {
  columns: Column<T>[];
  data: T[];
  keyExtractor: (item: T) => string;
  onRowClick?: (item: T) => void;
  className?: string;
}

export function DataTable<T>({ columns, data, keyExtractor, onRowClick, className }: DataTableProps<T>) {
  return (
    <Table className={className}>
      <TableHeader>
        <TableRow>
          {columns.map((col, index) => (
            <TableHead key={index} className={col.className}>
              {col.header}
            </TableHead>
          ))}
        </TableRow>
      </TableHeader>
      <TableBody>
        {data.map((item) => (
          <TableRow
            key={keyExtractor(item)}
            onClick={() => onRowClick && onRowClick(item)}
            className={onRowClick ? "cursor-pointer hover:bg-muted/50" : ""}
          >
            {columns.map((col, index) => (
              <TableCell key={index} className={col.className}>
                {col.cell ? col.cell(item) : (item[col.accessorKey!] as React.ReactNode)}
              </TableCell>
            ))}
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}
