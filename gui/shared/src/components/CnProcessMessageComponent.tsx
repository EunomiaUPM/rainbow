import React from "react";
import {
  MessageBody,
  MessageContent,
  MessageLog,
  MessageMeta,
  MessageMetaContainer,
  MessageTimestamp,
  MessageTitle,
  RoleHeader,
  RoleType,
} from "./ui/message";

type MessageComponentProps = {
  message: CNMessage;
};
let addSpacesFormat = (text: string) => {
  if (!text) return "";
  return text.replace(/(?!^)([A-Z])/g, " $1");
};

const MessageComponent: React.FC<MessageComponentProps> = ({message}) => {
  //   const {
  //   from,
  //   cn_message_id,
  //   cn_process_id,
  //   created_at,
  //   content,
  //   _type,
  // } = message;

  return (
    <MessageLog key={message.cn_message_id} variant={message.from as RoleType}>
      <RoleHeader from={message.from as RoleType}/>
      <MessageBody variant={message.from as RoleType}>
        <MessageTitle className="text-brand-sky mb-0 pb-0">
          {addSpacesFormat(message._type)}
        </MessageTitle>
        <MessageTimestamp created_at={message.created_at.toString()}/>
        <MessageMetaContainer>
          <MessageMeta label="Contract Message Id" value={message.cn_message_id.slice(9, 60)}/>
          <MessageMeta label="Contract Process Id" value={message.cn_process_id.slice(9, 60)}/>
        </MessageMetaContainer>
        <MessageContent content={JSON.stringify(message.content, null, 8)}/>
      </MessageBody>
    </MessageLog>
  );
};

export default MessageComponent;
