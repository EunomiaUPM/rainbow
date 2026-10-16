import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import React from "react";

dayjs.extend(relativeTime);

interface FormatDateProps {
  date: string | Date | undefined | null;
  format?: string;
}

export const FormatDate = ({ date, format = "DD/MM/YYYY - HH:mm" }: FormatDateProps) => {
  if (!date) return <span className="text-gray-700 dark:text-gray-400">-</span>;
  return <span>{dayjs(date).format(format)}</span>;
};

/** "3 minutes ago", with the exact time on hover. */
export const RelativeTime = ({ date, className }: { date: string | Date; className?: string }) => (
  <time
    dateTime={dayjs(date).toISOString()}
    title={dayjs(date).format("YYYY-MM-DD HH:mm:ss")}
    className={className}
  >
    {dayjs(date).fromNow()}
  </time>
);
