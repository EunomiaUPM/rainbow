import React from "react";
import dayjs from "dayjs";
// @ts-ignore
import { cn } from "@/lib/utils";
import { Badge } from "./ui/badge";
import Heading from "./ui/heading";
import { ListItem, ListItemKey } from "./ui/list";
// @ts-ignore

type MessageType = {
  from: "Provider" | "Consumer";
  _type: string;
  created_at: string;
  cn_message_id: string;
  cn_process_id: string;
  content: unknown;
};

type ContractMessageProps = {
  message: MessageType;
};
// Styles
const listItemPrimitiveResetClasses = " border-none p-0 m-0";

/**
 * Testeo para crear componente que sirva para los message containers
 * Esta función devuelve el componente del Message container del drawer
 * @param message // json del mensaje a renderizar
 * @returns HTMLElement del contenedor de
 */
export const CnProcessMessageContainer: React.FC<ContractMessageProps> = ({
  message,
}) => {
  const isProvider = message.from === "Provider";
  const baseColor = isProvider ? "provider" : "consumer";
  let addSpacesFormat = (text: string) => {
    return text.replace(/(?!^)([A-Z])/g, " $1");
  };

  return (
    <div
      className={cn(
        "my-4 text-sm overflow-hidden",
        isProvider ? "pr-12" : "pl-12"
      )}
    >
      {/* Header */}
      <div
        className={cn(
          "flex w-full",
          isProvider ? "justify-start" : "justify-end"
        )}
      >
        <div
          className={cn(
            "uppercase text-18 px-2 font-medium rounded-t-sm",
            `bg-roles-${baseColor}/20 text-roles-${baseColor}`,
            isProvider ? "ml-1" : "mr-1"
          )}
        >
          {message.from}
        </div>
      </div>

      {/* Message Body */}
      <div
        className={cn(
          "w-full px-4 py-3 rounded-md rounded-b-xl border flex flex-col gap-2",
          `bg-roles-${baseColor}/10 border-roles-${baseColor}/50`
        )}
      >
        {/* Type */}
        <Heading level="h5" className=" mb-0 text-20 ">
          {addSpacesFormat(message._type)}{" "}
        </Heading>

        {/* Timestamp */}
        <p className="text-gray-100/50 mb-3">
          <i>{dayjs(message.created_at).format("DD/MM/YYYY - HH:mm")}</i>
        </p>

        {/* IDs */}
        <MessageRow
          label="Contract Message Id"
          value={message.cn_message_id.slice(9, 60)}
        />
        <MessageRow
          label="Contract Process Id"
          value={message.cn_process_id.slice(9, 60)}
        />

        {/* Content */}
        <div className="flex flex-col gap-3">
          <p className="font-bold min-w-40 text-white/60">Content:</p>
          <div className="w-full break-all">
            <pre className="p-4 rounded-lg break-all text-[11px] bg-black/70 text-secondary-400">
              <code className="whitespace-pre-wrap break-all">
                {JSON.stringify(message.content, null, 2)}
              </code>
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
};

const MessageRow = ({ label, value }: { label: string; value: string }) => (
  <ListItem
    className={"flex flex-row flex-wrap gap-1 mb-1 text-white/70" + listItemPrimitiveResetClasses}
  >
    <ListItemKey className="font-bold max-w-40">{label}</ListItemKey>
    <Badge variant="info">{value}</Badge>
  </ListItem>
);
