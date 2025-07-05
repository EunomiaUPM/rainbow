import React from "react";
import {
  MessageLog,
  RoleHeader,
  MessageBody,
  MessageTitle,
  MessageTimestamp,
  MessageMeta,
  MessageMetaContainer,
  MessageContent,
} from "./ui/message";

type MessageComponentProps = {
  message: CNMessage;
};
let addSpacesFormat = (text: string) => {
  if (!text) return "";
  return text.replace(/(?!^)([A-Z])/g, " $1");
};

const MessageComponent: React.FC<MessageComponentProps> = ({ message }) => {
  //   const {
  //   from,
  //   cn_message_id,
  //   cn_process_id,
  //   created_at,
  //   content,
  //   _type,
  // } = message;
  
  return (
    <MessageLog key={message.cn_message_id} variant={message.from}>
      <RoleHeader from={message.from} />
      <MessageBody variant={message.from}>
        <MessageTitle className="text-brand-sky mb-0 pb-0">
          {addSpacesFormat(message._type)}
        </MessageTitle>
        <MessageTimestamp created_at={message.created_at} />
        <MessageMetaContainer>
          <MessageMeta
            label="Contract Message Id"
            value={message.cn_message_id.slice(9, 60)}
          />
          <MessageMeta
            label="Contract Process Id"
            value={message.cn_process_id.slice(9, 60)}
          />
        </MessageMetaContainer>
        <MessageContent content={message.content} />
      </MessageBody>
    </MessageLog>
  );
};

export default MessageComponent;
