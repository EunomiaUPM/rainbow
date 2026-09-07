/**
 * DataTable.tsx
 *
 * Enterprise-grade generic table component with built-in client-side
 * sorting, live search filtering, pagination, and status indicators.
 */

import React, { useState, useMemo } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./ui/table";
import { Input } from "./ui/input";
import { Badge } from "./ui/badge";
import { Button } from "./ui/button";
import {
  ArrowDown,
  ArrowUp,
  ArrowUpDown,
  ChevronLeft,
  ChevronRight,
  Inbox,
  SearchX,
  X,
} from "lucide-react";
import { cn } from "shared/src/lib/utils";

// =============================================================================
// TYPES
// =============================================================================

/**
 * Column definition for the DataTable.
 *
 * @template T - The type of data items in the table
 */
export interface Column<T> {
  /** Header text or component displayed in the table header */
  header: React.ReactNode | string;

  /**
   * Key to access the value from the data item.
   * Used when no custom cell renderer is provided.
   */
  accessorKey?: keyof T;

  /**
   * Custom cell renderer function.
   * Takes precedence over accessorKey when provided.
   */
  cell?: (item: T) => React.ReactNode;

  /** Additional CSS class for the cells */
  className?: string;

  /** Additional CSS class for the header cell */
  headerClassName?: string;

  /** Whether this column is sortable. Defaults to true for string headers or when accessorKey/sortValue is set */
  sortable?: boolean;

  /** Custom sort key name */
  sortKey?: string;

  /** Custom sort value extractor */
  sortValue?: (item: T) => string | number | boolean | null | undefined;

  /** Whether this column should be searched during filtering. Defaults to true */
  searchable?: boolean;

  /** Custom search text extractor for this column */
  searchValue?: (item: T) => string;
}

/**
 * Props for the DataTable component.
 *
 * @template T - The type of data items in the table
 */
export interface DataTableProps<T> {
  /** Column definitions */
  columns: Column<T>[];

  /** Array of data items to display */
  data: T[];

  /** Function to extract a unique key from each item */
  keyExtractor: (item: T) => string;

  /** Optional handler for row clicks */
  onRowClick?: (item: T) => void;

  /** Additional CSS class for the table */
  className?: string;

  /** Additional CSS class for the table container */
  containerClassName?: string;

  /**
   * Message shown when `data` is empty. Defaults to a generic
   * "No records found". Pass a domain-specific phrase for clarity.
   */
  emptyMessage?: React.ReactNode;

  /** Table title displayed in the toolbar */
  title?: React.ReactNode;

  /** Subtitle displayed under the title */
  subtitle?: React.ReactNode;

  /** Extra actions/buttons displayed on the toolbar */
  toolbarActions?: React.ReactNode;

  /** Whether client-side search is enabled. Defaults to true if data is non-empty. */
  searchable?: boolean;

  /** Custom placeholder for search input */
  searchPlaceholder?: string;

  /** Custom filter function */
  filterFn?: (item: T, query: string) => boolean;

  /** Default column key to sort by */
  defaultSortKey?: string;

  /** Default sort direction */
  defaultSortDirection?: "asc" | "desc";

  /** Optional items per page for client-side pagination */
  pageSize?: number;

  /** Whether to hide the search and filter toolbar */
  hideToolbar?: boolean;

  /** Whether to hide the record count badge */
  hideCount?: boolean;
}

// =============================================================================
// COMPONENT
// =============================================================================

/**
 * Enterprise table component with built-in client-side sorting, filtering, and pagination.
 */
export function DataTable<T extends Record<string, any> = any>({
  columns,
  data,
  keyExtractor,
  onRowClick,
  className,
  containerClassName,
  emptyMessage,
  title,
  subtitle,
  toolbarActions,
  searchable = true,
  searchPlaceholder = "Filter records...",
  filterFn,
  defaultSortKey,
  defaultSortDirection,
  pageSize,
  hideToolbar = false,
  hideCount = false,
}: DataTableProps<T>) {
  const [searchQuery, setSearchQuery] = useState("");
  const [sortConfig, setSortConfig] = useState<{
    key: string;
    direction: "asc" | "desc";
  } | null>(
    defaultSortKey ? { key: defaultSortKey, direction: defaultSortDirection ?? "asc" } : null,
  );
  const [currentPage, setCurrentPage] = useState(1);
  const [itemsPerPage, setItemsPerPage] = useState<number | undefined>(pageSize);

  // Client-side live filtering
  const filteredData = useMemo(() => {
    const trimmed = searchQuery.trim().toLowerCase();
    if (!trimmed) return data;

    const terms = trimmed.split(/\s+/).filter(Boolean);

    return data.filter((item) => {
      if (filterFn) return filterFn(item, trimmed);

      // Check each column's searchable value
      for (const col of columns) {
        if (col.searchable === false) continue;

        let valStr = "";
        if (col.searchValue) {
          valStr = col.searchValue(item).toLowerCase();
        } else if (col.accessorKey) {
          const raw = item[col.accessorKey];
          if (raw !== null && raw !== undefined) {
            valStr = String(raw).toLowerCase();
          }
        }

        if (valStr && terms.every((t) => valStr.includes(t))) {
          return true;
        }
      }

      // Fallback: check all string/number fields of the item
      const itemValues = Object.values(item)
        .filter((v) => typeof v === "string" || typeof v === "number")
        .map((v) => String(v).toLowerCase())
        .join(" ");

      return terms.every((t) => itemValues.includes(t));
    });
  }, [data, searchQuery, filterFn, columns]);

  // Client-side column sorting
  const sortedData = useMemo(() => {
    if (!sortConfig) return filteredData;

    const col = columns.find(
      (c, index) =>
        (c.sortKey ??
          (c.accessorKey as string) ??
          (typeof c.header === "string" ? c.header : `col_${index}`)) === sortConfig.key,
    );

    return [...filteredData].sort((a, b) => {
      let aVal: any;
      let bVal: any;

      if (col?.sortValue) {
        aVal = col.sortValue(a);
        bVal = col.sortValue(b);
      } else if (col?.accessorKey) {
        aVal = a[col.accessorKey];
        bVal = b[col.accessorKey];
      } else {
        aVal = a[sortConfig.key];
        bVal = b[sortConfig.key];
      }

      if (aVal === bVal) return 0;
      if (aVal === null || aVal === undefined) return sortConfig.direction === "asc" ? 1 : -1;
      if (bVal === null || bVal === undefined) return sortConfig.direction === "asc" ? -1 : 1;

      if (typeof aVal === "number" && typeof bVal === "number") {
        return sortConfig.direction === "asc" ? aVal - bVal : bVal - aVal;
      }

      if (typeof aVal === "boolean" && typeof bVal === "boolean") {
        return sortConfig.direction === "asc"
          ? aVal === bVal
            ? 0
            : aVal
              ? 1
              : -1
          : aVal === bVal
            ? 0
            : aVal
              ? -1
              : 1;
      }

      const aStr = String(aVal).toLowerCase();
      const bStr = String(bVal).toLowerCase();
      return sortConfig.direction === "asc" ? aStr.localeCompare(bStr) : bStr.localeCompare(aStr);
    });
  }, [filteredData, sortConfig, columns]);

  // Reset page when search changes
  React.useEffect(() => {
    setCurrentPage(1);
  }, [searchQuery]);

  // Client-side pagination
  const totalPages = itemsPerPage ? Math.ceil(sortedData.length / itemsPerPage) || 1 : 1;
  const paginatedData = useMemo(() => {
    if (!itemsPerPage) return sortedData;
    const start = (currentPage - 1) * itemsPerPage;
    return sortedData.slice(start, start + itemsPerPage);
  }, [sortedData, currentPage, itemsPerPage]);

  const handleHeaderClick = (col: Column<T>, index: number) => {
    // Only sort if sortable
    const isStringHeader = typeof col.header === "string";
    const isSortable =
      col.sortable !== undefined
        ? col.sortable
        : col.sortKey !== undefined ||
          col.sortValue !== undefined ||
          (col.accessorKey !== undefined && isStringHeader);

    if (!isSortable) return;

    const colKey =
      col.sortKey ??
      (col.accessorKey as string) ??
      (isStringHeader ? (col.header as string) : `col_${index}`);

    if (sortConfig?.key === colKey) {
      if (sortConfig.direction === "asc") {
        setSortConfig({ key: colKey, direction: "desc" });
      } else {
        setSortConfig(null);
      }
    } else {
      setSortConfig({ key: colKey, direction: "asc" });
    }
  };

  const showToolbar = !hideToolbar && (searchable || title || toolbarActions || !hideCount);
  const isFiltered = searchQuery.trim().length > 0;

  return (
    <div className="flex flex-col gap-2.5 w-full">
      {/* Table Toolbar */}
      {showToolbar && (
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 px-1 py-1">
          <div className="flex flex-wrap items-center gap-3">
            {title && (
              <div className="flex flex-col">
                <h3 className="text-sm font-semibold text-brand-snow">{title}</h3>
                {subtitle && <p className="text-xs text-muted-foreground">{subtitle}</p>}
              </div>
            )}

            {searchable && (
              <div className="w-full sm:w-64 max-w-sm">
                <Input
                  type="search"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  onClear={() => setSearchQuery("")}
                  placeholder={searchPlaceholder}
                  className="text-xs"
                />
              </div>
            )}

            {!hideCount && (
              <div className="flex items-center gap-1.5">
                <Badge
                  variant={isFiltered ? "info" : "default"}
                  size="sm"
                  className="font-mono text-xs"
                >
                  {isFiltered
                    ? `${filteredData.length} of ${data.length}`
                    : `${data.length} records`}
                </Badge>
                {isFiltered && (
                  <Button
                    variant="ghost"
                    size="xs"
                    onClick={() => setSearchQuery("")}
                    className="text-muted-foreground hover:text-ink gap-1"
                  >
                    <X className="h-3 w-3" />
                    Clear
                  </Button>
                )}
              </div>
            )}
          </div>

          {toolbarActions && (
            <div className="flex items-center gap-2 shrink-0">{toolbarActions}</div>
          )}
        </div>
      )}

      {/* Main Table */}
      <Table className={className} containerClassName={containerClassName}>
        <TableHeader className="sticky top-0 z-10 bg-background-800/90 backdrop-blur border-b border-ink/10">
          <TableRow>
            {columns.map((col, index) => {
              const isStringHeader = typeof col.header === "string";
              const isSortable =
                col.sortable !== undefined
                  ? col.sortable
                  : col.sortKey !== undefined ||
                    col.sortValue !== undefined ||
                    (col.accessorKey !== undefined && isStringHeader);

              const colKey =
                col.sortKey ??
                (col.accessorKey as string) ??
                (isStringHeader ? (col.header as string) : `col_${index}`);

              const isSortedAsc = sortConfig?.key === colKey && sortConfig.direction === "asc";
              const isSortedDesc = sortConfig?.key === colKey && sortConfig.direction === "desc";

              return (
                <TableHead
                  key={index}
                  className={cn(
                    isSortable &&
                      "cursor-pointer select-none hover:text-ink transition-colors group",
                    col.className,
                    col.headerClassName,
                  )}
                  onClick={() => handleHeaderClick(col, index)}
                >
                  <div className="inline-flex items-center gap-1.5">
                    <span>{col.header}</span>
                    {isSortable && (
                      <span className="shrink-0">
                        {isSortedAsc ? (
                          <ArrowUp className="h-3.5 w-3.5 text-brand-sky" />
                        ) : isSortedDesc ? (
                          <ArrowDown className="h-3.5 w-3.5 text-brand-sky" />
                        ) : (
                          <ArrowUpDown className="h-3.5 w-3.5 opacity-25 group-hover:opacity-70 transition-opacity" />
                        )}
                      </span>
                    )}
                  </div>
                </TableHead>
              );
            })}
          </TableRow>
        </TableHeader>

        <TableBody>
          {paginatedData.length === 0 ? (
            <TableRow className="hover:bg-transparent">
              <TableCell colSpan={columns.length} className="text-center py-12">
                {isFiltered ? (
                  <div className="flex flex-col items-center justify-center gap-2">
                    <div className="p-2.5 rounded-full bg-ink/5 border border-ink/10 text-muted-foreground">
                      <SearchX className="h-5 w-5" />
                    </div>
                    <span className="text-xs font-medium text-foreground">
                      No matching records found
                    </span>
                    <span className="text-xs text-muted-foreground">
                      No items matched &quot;{searchQuery}&quot;
                    </span>
                    <Button
                      variant="outline"
                      size="xs"
                      onClick={() => setSearchQuery("")}
                      className="mt-2"
                    >
                      Reset filter
                    </Button>
                  </div>
                ) : (
                  <div className="flex flex-col items-center justify-center gap-2">
                    <div className="p-2.5 rounded-full bg-ink/5 border border-ink/10 text-muted-foreground/50">
                      <Inbox className="h-5 w-5" />
                    </div>
                    <span className="text-xs text-muted-foreground italic">
                      {emptyMessage ?? "Nothing here yet"}
                    </span>
                  </div>
                )}
              </TableCell>
            </TableRow>
          ) : (
            paginatedData.map((item) => (
              <TableRow
                key={keyExtractor(item)}
                onClick={() => onRowClick && onRowClick(item)}
                className={cn(
                  onRowClick &&
                    "cursor-pointer hover:bg-ink/[0.04] active:bg-ink/[0.06] transition-colors",
                )}
              >
                {columns.map((col, index) => (
                  <TableCell
                    key={index}
                    className={cn("text-ink [&>p]:text-ink", col.className)}
                  >
                    {col.cell ? col.cell(item) : (item[col.accessorKey!] as React.ReactNode)}
                  </TableCell>
                ))}
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>

      {/* Pagination Footer (if enabled) */}
      {itemsPerPage && totalPages > 1 && (
        <div className="flex items-center justify-between px-2 py-1.5 text-xs text-muted-foreground border-t border-ink/5">
          <span>
            Page {currentPage} of {totalPages} ({sortedData.length} total)
          </span>
          <div className="flex items-center gap-1.5">
            <Button
              variant="outline"
              size="xs"
              disabled={currentPage <= 1}
              onClick={() => setCurrentPage((p) => Math.max(1, p - 1))}
              className="gap-1"
            >
              <ChevronLeft className="h-3 w-3" />
              Prev
            </Button>
            <Button
              variant="outline"
              size="xs"
              disabled={currentPage >= totalPages}
              onClick={() => setCurrentPage((p) => Math.min(totalPages, p + 1))}
              className="gap-1"
            >
              Next
              <ChevronRight className="h-3 w-3" />
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
