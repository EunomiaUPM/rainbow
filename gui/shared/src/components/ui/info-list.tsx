import React from "react";
import { Badge, BadgeState, BadgeRole } from "./badge";
import dayjs from "dayjs";
import { cn } from "shared/src/lib/utils";
import { formatUrn } from "shared/src/lib/utils";

export type InfoItemValue =
  | string
  | undefined
  | null
  | { type: "date"; value: string | Date }
  | { type: "status"; value: string }
  | { type: "role"; value: string }
  | { type: "urn"; value: string | undefined }
  | { type: "custom"; content: React.ReactNode };

export interface InfoItemProps {
  label: string;
  value: InfoItemValue;
  className?: string; // class for the container
  keyClassName?: string; // class for the key
}

/**
 * Renders a list of information items with consistent styling.
 */
export const InfoList = ({
  items,
  className,
}: {
  items: InfoItemProps[];
  className?: string;
}) => {
  return (
    <div className={cn("min-w-full px-0", className)}>
      {items.map((item, index) => (
        <InfoListItem key={index} {...item} />
      ))}
    </div>
  );
};

const InfoListItem = ({ label, value, className, keyClassName }: InfoItemProps) => {
  if (value === undefined || value === null) return null;

  const renderValue = () => {
    if (typeof value === "string") {
      return <Badge variant="info">{value}</Badge>;
    }

    if (typeof value === "object") {
      if ("content" in value) return value.content;

      switch (value.type) {
        case "date":
          return <p>{dayjs(value.value).format("DD/MM/YY HH:mm")}</p>;
        case "status":
          return (
            <Badge variant="status" state={value.value as BadgeState}>
              {value.value}
            </Badge>
          );
        case "role":
          return (
            <Badge variant="role" role={value.value as BadgeRole}>
              {value.value}
            </Badge>
          );
        case "urn":
          return <Badge variant="info">{formatUrn(value.value)}</Badge>;
        default:
          return null;
      }
    }
    return null;
  };

  return (
    <div className={cn("flex flex-col py-2 border-b border-white/5 last:border-0", className)}>
      <span className={cn("text-[10px] uppercase tracking-wide text-white/50 font-medium mb-1", keyClassName)}>{label}</span>
      <div className="text-sm font-medium text-white/90">{renderValue()}</div>
    </div>
  );
};
