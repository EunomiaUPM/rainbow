import React from "react";
// @ts-ignore
import { cn } from "@/lib/utils";
// @ts-ignore
import {
  MessageLog,
  RoleHeader,
  MessageBody,
  MessageTitle,
  MessageTimestamp,
  MessageMeta,
  MessageContent,
  type MessageType,
} from "./ui/message";

type MessageComponentProps = {
  message: MessageType;
};
let addSpacesFormat = (text: string) => {
    if (!text) return "";
  return text.replace(/(?!^)([A-Z])/g, " $1");
};

const MessageComponent: React.FC<MessageComponentProps> = ({ message }) => {
  return (
    <MessageLog key={message.cn_message_id} variant={message.from}>
      <RoleHeader from={message.from} />
      <MessageBody variant={message.from}>
        <MessageTitle className="text-brand-sky mb-0 pb-0">
          {addSpacesFormat(message._type)}
        </MessageTitle>
        <MessageTimestamp created_at={message.created_at} />
        <MessageMeta
          label="Contract Message Id"
          value={message.cn_message_id.slice(9, 60)}
        />
        <MessageMeta
          label="Contract Process Id"
          value={message.cn_process_id.slice(9, 60)}
        />
        <MessageContent content={message.content} />
      </MessageBody>
    </MessageLog>
  );
};

export default MessageComponent;
