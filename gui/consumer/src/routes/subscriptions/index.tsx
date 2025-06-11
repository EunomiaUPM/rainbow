import {createFileRoute} from "@tanstack/react-router";
import {getCatalogsOptions} from "shared/src/data/catalog-queries.ts";
import {useGetNotificationsBySubscriptionId, useGetSubscriptionById} from "shared/src/data/pubsub-queries.ts";
import {PubSubContext} from "shared/src/context/PubSubContext.tsx";
import {useContext} from "react";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from "shared/src/components/ui/table";
import dayjs from "dayjs";

const RouteComponent = () => {
    const {subscriptionId} = useContext(PubSubContext)!;
    const {data: subscription} = useGetSubscriptionById(subscriptionId!);
    const {data: notifications} = useGetNotificationsBySubscriptionId(subscriptionId!);
    return (
        <div className="space-y-4">
            <h1 className="text-xl font-bold">Subscription</h1>
            <div>
                Subscription with id : {subscription.subscriptionId}
            </div>
            <div>

                <h2>Main Catalog info: </h2>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Key</TableHead>
                            <TableHead>Value</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>Subscription callback address</TableCell>
                            <TableCell>{subscription.callbackAddress}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Subscription state</TableCell>
                            <TableCell>{subscription.active ? "ACTIVE" : "INACTIVE"}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Subscription entity</TableCell>
                            <TableCell>{subscription.subscriptionEntity}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Subscription creation date</TableCell>
                            <TableCell>
                                {dayjs(subscription.timestamp).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Subscription expiration date</TableCell>
                            <TableCell>
                                {dayjs(subscription.expirationTime).format("DD/MM/YYYY - HH:mm")}
                            </TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </div>

            <div>
                <h2>Notifications</h2>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Notification Id</TableHead>
                            <TableHead>Category</TableHead>
                            <TableHead>Subcategory</TableHead>
                            <TableHead>Message Type</TableHead>
                            <TableHead>Message operation</TableHead>
                            <TableHead>Message content</TableHead>
                            <TableHead>Message timestamp</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {notifications.map(notification => (
                            <TableRow key={notification.notificationId.slice(0, 20)}>
                                <TableCell>
                                    {notification.notificationId.slice(0, 20) + "..."}
                                </TableCell>
                                <TableCell>{notification.category}</TableCell>
                                <TableCell>{notification.subcategory}</TableCell>
                                <TableCell>{notification.messageType}</TableCell>
                                <TableCell>{notification.messageOperation}</TableCell>
                                <TableCell>{JSON.stringify(notification.messageContent)}</TableCell>
                                <TableCell>
                                    {dayjs(notification.timestamp).format("DD/MM/YYYY - HH:mm")}
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>
        </div>
    );
};

export const Route = createFileRoute("/subscriptions/")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
    loader: async ({context: {queryClient}}) => {
        return await queryClient.ensureQueryData(getCatalogsOptions());
    },
});
