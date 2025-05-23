import {createFileRoute} from '@tanstack/react-router'

export const Route = createFileRoute('/subscriptions')({
    component: RouteComponent,
})

function RouteComponent() {
    return <div>Hello "/subscriptions"!</div>
}
