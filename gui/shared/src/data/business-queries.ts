import {useMutation, useQueryClient} from "@tanstack/react-query";

/**
 *  POST /datasets/{datasetId}/policies
 * */
interface LoginPayload {
    api_gateway: string;
    content: {
        authRequestId: string
    }
}

export const postLogin = async ({api_gateway, content}: LoginPayload) => {
    const login: string = await (
        await fetch(api_gateway + `/login`, {
            method: "POST",
            body: JSON.stringify(content),
            headers: {
                "Content-Type": "application/json",
            },
        })
    ).text();
    return login;
}

export const usePostLogin = () => {
    const queryClient = useQueryClient()
    const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
        mutationFn: postLogin,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async (_data, variables) => {
            console.log("onSuccess")
            // @ts-ignore
            // refetch auth data...
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}