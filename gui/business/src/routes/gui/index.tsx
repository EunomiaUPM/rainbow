import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/gui/')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/gui/"!</div>
}
