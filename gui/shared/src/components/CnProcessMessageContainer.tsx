import React from "react";
import dayjs from "dayjs";
// @ts-ignore
import { cn } from "@/lib/utils";
import { Badge } from "./ui/badge";
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
    <div className={cn("my-4 text-sm", isProvider ? "pr-12" : "pl-12")}>
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
          "w-full px-4 py-3 rounded-md rounded-b-xl border",
          `bg-roles-${baseColor}/10 border-roles-${baseColor}/50`
        )}
      >
        {/* Type */}
        <div className="text-20">{addSpacesFormat(message._type)} </div>

        {/* Timestamp */}
        <p className="text-gray-100/50 mb-3">
          <i>{dayjs(message.created_at).format("DD/MM/YYYY - HH:mm")}</i>
        </p>

        {/* IDs */}
        {/* <MessageRow
          label="Contract Message Id"
          value={message.cn_message_id.slice(9, 60)}
        /> */}
        <div
          className="flex gap-3 mb-1  text-white/70 "
          key={message.cn_message_id.slice(0, 20)}
        >
          <p className="font-bold  min-w-40  ">Contract Message Id</p>
          <Badge variant={"info"}>{message.cn_message_id.slice(9, 60)}</Badge>
        </div>
        {/* <MessageRow
          label="Contract Process Id"
          value={message.cn_process_id.slice(9, 60)}
        /> */}
        <div
          className="flex gap-3 mb-1  text-white/70 "
          key={message.cn_process_id.slice(0, 20)}
        >
          <p className="font-bold  min-w-40  ">Contract Process Id</p>
          <Badge variant={"info"}>{message.cn_process_id.slice(9, 60)}</Badge>
        </div>

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
  <div className="flex gap-3 mb-1 text-white/70">
    <p className="font-bold min-w-40">{label}</p>
    <p className="w-full">{value}</p>
  </div>
);
