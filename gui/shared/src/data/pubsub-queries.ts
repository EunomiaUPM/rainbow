import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

/**
 *  GET /catalog/subscriptions
 * */
export const getSubscriptions = async (api_gateway: string) => {
    const catalogs: Subscription[] = await (
        await fetch(api_gateway + "/subscriptions")
    ).json();
    return catalogs;
}

export const getSubscriptionsOptions = (api_gateway: string) => queryOptions({
    queryKey: ["SUBSCRIPTIONS"],
    queryFn: () => getSubscriptions(api_gateway)
})

export const useGetAgreements = () => {
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext);
    const {data, isLoading, isError, error} = useSuspenseQuery(getSubscriptionsOptions(api_gateway))
    return {data, isLoading, isError, error}
}


/**
 *  GET /catalog/subscriptions/{subscriptionId}
 * */
export const getSubscriptionById = async (api_gateway: string, subscriptionId: UUID) => {
    const catalogs: Subscription = await (
        await fetch(api_gateway + `/subscriptions/${subscriptionId}`)
    ).json();
    return catalogs;
}

export const getSubscriptionByIdOptions = (api_gateway: string, subscriptionId: UUID) => queryOptions({
    queryKey: ["SUBSCRIPTIONS_BY_ID", subscriptionId],
    queryFn: () => getSubscriptionById(api_gateway, subscriptionId)
})

export const useGetSubscriptionById = (subscriptionId: UUID) => {
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext);
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getSubscriptionByIdOptions(api_gateway, subscriptionId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalog/subscriptions/callback-url/{callbackUrl}
 * */
export const getSubscriptionByCallbackAddress = async (api_gateway: string, callbackUrl: string) => {
    const catalogs: Subscription = await (
        await fetch(api_gateway + `/subscriptions?callback_address=${encodeURIComponent(callbackUrl)}`)
    ).json();
    return catalogs;
}

export const getSubscriptionByCallbackAddressOptions = (api_gateway: string, callbackUrl: string) => queryOptions({
    queryKey: ["SUBSCRIPTIONS_BY_CALLBACK_URL", callbackUrl],
    queryFn: () => getSubscriptionByCallbackAddress(api_gateway, callbackUrl)
})

export const useGetSubscriptionByCallbackAddress = (callbackUrl: string) => {
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext);
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getSubscriptionByCallbackAddressOptions(api_gateway, callbackUrl))
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalog/subscriptions/{subscriptionId}/notifications
 * */
export const getNotificationsBySubscriptionId = async (api_gateway: string, subscriptionId: UUID) => {
    const catalogs: NotificationSub[] = await (
        await fetch(api_gateway + `/subscriptions/${subscriptionId}/notifications`)
    ).json();
    return catalogs;
}

export const getNotificationsBySubscriptionIdOptions = (api_gateway: string, subscriptionId: string) => queryOptions({
    queryKey: ["NOTIFICATIONS_BY_SUBSCRIPTION_ID", subscriptionId],
    queryFn: () => getNotificationsBySubscriptionId(api_gateway, subscriptionId)
})

export const useGetNotificationsBySubscriptionId = (subscriptionId: string) => {
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext);
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getNotificationsBySubscriptionIdOptions(api_gateway, subscriptionId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalog/subscriptions/{subscriptionId}/notifications-pending
 * */
export const getPendingNotificationsBySubscriptionId = async (api_gateway: string, subscriptionId: UUID) => {
    const catalogs: NotificationSub[] = await (
        await fetch(api_gateway + `/subscriptions/${subscriptionId}/notifications-pending`)
    ).json();
    return catalogs;
}

export const getPendingNotificationsBySubscriptionIdOptions = (api_gateway: string, subscriptionId: string) => queryOptions({
    queryKey: ["PENDING_NOTIFICATIONS_BY_SUBSCRIPTION_ID", subscriptionId],
    queryFn: () => getPendingNotificationsBySubscriptionId(api_gateway, subscriptionId)
})

export const useGetPendingNotificationsBySubscriptionId = (subscriptionId: string) => {
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext);
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getPendingNotificationsBySubscriptionIdOptions(api_gateway, subscriptionId))
    return {data, isLoading, isError, error}
}

