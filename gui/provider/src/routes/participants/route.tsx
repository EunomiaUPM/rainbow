import {createFileRoute} from '@tanstack/react-router'

export const Route = createFileRoute('/participants')({
    component: RouteComponent,
})

function RouteComponent() {
    return <div>Hello "/participants"!</div>
}
