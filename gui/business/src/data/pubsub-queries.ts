import {GATEWAY_API} from "@/data/index.ts";
import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";

/**
 *  GET /catalog/subscriptions
 * */
export const getSubscriptions = async () => {
    const catalogs: Subscription[] = await (
        await fetch(GATEWAY_API + "/subscriptions")
    ).json();
    return catalogs;
}

export const getSubscriptionsOptions = () => queryOptions({
    queryKey: ["SUBSCRIPTIONS"],
    queryFn: getSubscriptions
})

export const useGetAgreements = () => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getSubscriptionsOptions())
    return {data, isLoading, isError, error}
}


/**
 *  GET /catalog/subscriptions/{subscriptionId}
 * */
export const getSubscriptionById = async (subscriptionId: UUID) => {
    const catalogs: Subscription = await (
        await fetch(GATEWAY_API + `/subscriptions/${subscriptionId}`)
    ).json();
    return catalogs;
}

export const getSubscriptionByIdOptions = (subscriptionId: UUID) => queryOptions({
    queryKey: ["SUBSCRIPTIONS_BY_ID", subscriptionId],
    queryFn: () => getSubscriptionById(subscriptionId)
})

export const useGetSubscriptionById = (subscriptionId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getSubscriptionByIdOptions(subscriptionId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalog/subscriptions/callback-url/{callbackUrl}
 * */
export const getSubscriptionByCallbackAddress = async (callbackUrl: string) => {
    const catalogs: Subscription = await (
        await fetch(GATEWAY_API + `/subscriptions?callback_address=${encodeURIComponent(callbackUrl)}`)
    ).json();
    return catalogs;
}

export const getSubscriptionByCallbackAddressOptions = (callbackUrl: string) => queryOptions({
    queryKey: ["SUBSCRIPTIONS_BY_CALLBACK_URL", callbackUrl],
    queryFn: () => getSubscriptionByCallbackAddress(callbackUrl)
})

export const useGetSubscriptionByCallbackAddress = (callbackUrl: string) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getSubscriptionByCallbackAddressOptions(callbackUrl))
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalog/subscriptions/{subscriptionId}/notifications
 * */
export const getNotificationsBySubscriptionId = async (subscriptionId: UUID) => {
    const catalogs: NotificationSub[] = await (
        await fetch(GATEWAY_API + `/subscriptions/${subscriptionId}/notifications`)
    ).json();
    return catalogs;
}

export const getNotificationsBySubscriptionIdOptions = (subscriptionId: string) => queryOptions({
    queryKey: ["NOTIFICATIONS_BY_SUBSCRIPTION_ID", subscriptionId],
    queryFn: () => getNotificationsBySubscriptionId(subscriptionId)
})

export const useGetNotificationsBySubscriptionId = (subscriptionId: string) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getNotificationsBySubscriptionIdOptions(subscriptionId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalog/subscriptions/{subscriptionId}/notifications-pending
 * */
export const getPendingNotificationsBySubscriptionId = async (subscriptionId: UUID) => {
    const catalogs: NotificationSub[] = await (
        await fetch(GATEWAY_API + `/subscriptions/${subscriptionId}/notifications-pending`)
    ).json();
    return catalogs;
}

export const getPendingNotificationsBySubscriptionIdOptions = (subscriptionId: string) => queryOptions({
    queryKey: ["PENDING_NOTIFICATIONS_BY_SUBSCRIPTION_ID", subscriptionId],
    queryFn: () => getPendingNotificationsBySubscriptionId(subscriptionId)
})

export const useGetPendingNotificationsBySubscriptionId = (subscriptionId: string) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getPendingNotificationsBySubscriptionIdOptions(subscriptionId))
    return {data, isLoading, isError, error}
}

