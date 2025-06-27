import {createFileRoute, Outlet} from '@tanstack/react-router'

export const Route = createFileRoute('/login')({
    component: RouteComponent,
})

function RouteComponent() {
    return <div><Outlet/></div>
}
