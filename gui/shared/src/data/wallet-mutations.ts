import {useMutation, useQueryClient} from "@tanstack/react-query";

/**
 *  POST /wallet/onboard
 * */
interface WalletOnboarPayload {
    api_gateway: string;
}

export const postWalletOnboard = async ({api_gateway,}: WalletOnboarPayload) => {
    await fetch(api_gateway + `/wallet/onboard`, {
        method: "POST",
    })
}

export const useWalletOnboard = () => {
    const queryClient = useQueryClient()
    const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
        mutationFn: postWalletOnboard,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async (_data, variables) => {
            console.log("onSuccess")
            // @ts-ignore
            await queryClient.refetchQueries(["PARTICIPANTS"]);
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}
