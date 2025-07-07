import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/contract-negotiation/')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/contract-negotiation/"!</div>
}
