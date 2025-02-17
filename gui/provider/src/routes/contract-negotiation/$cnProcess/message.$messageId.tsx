import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute(
  '/contract-negotiation/$cnProcess/message/$messageId',
)({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/contract-negotiation/$cnProcess/message/messageId"!</div>
}
