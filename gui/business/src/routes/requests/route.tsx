import {createFileRoute, Outlet, redirect} from '@tanstack/react-router'

export const Route = createFileRoute('/requests')({
    component: RouteComponent,
    beforeLoad: ({context}) => {
        // @ts-ignore
        if (!context.auth.isAuthenticated) {
            throw redirect({
                to: '/',
            });
        }
    }
})

function RouteComponent() {
    return <div><Outlet/></div>
}
