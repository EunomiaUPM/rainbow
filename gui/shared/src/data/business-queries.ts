import {useMutation, useQuery, useQueryClient} from "@tanstack/react-query";

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


export const postLoginPoll = async ({api_gateway, content}: LoginPayload) => {
    const login: string = await (
        await fetch(api_gateway + `/login/poll`, {
            method: "POST",
            body: JSON.stringify(content),
            headers: {
                "Content-Type": "application/json",
            },
        })
    ).text();
    return login;
}

export const usePostLoginPoll = ({api_gateway, content}: {
    api_gateway: string;
    content: { authRequestId: string }
}) => {
    return useQuery({
        queryKey: ['loginPoll', content.authRequestId],
        queryFn: async () => {
            const login: string = await (
                await fetch(api_gateway + `/login/poll`, {
                    method: "POST", // Sigue siendo POST si lo necesitas así
                    body: JSON.stringify(content),
                    headers: {
                        "Content-Type": "application/json",
                    },
                })
            ).text();
            return login;
        },
        // Estas opciones se pasarán desde el `usePostLoginPollQuery` de arriba
        // para manejar el `enabled` y `refetchInterval`.
    });
};


// export const usePostLoginPoll = () => {
//     const queryClient = useQueryClient()
//     const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
//         mutationFn: postLoginPoll,
//         onMutate: async () => {
//         },
//         onError: (error) => {
//             console.log("onError")
//             console.log(error)
//         },
//         onSuccess: async (_data, variables) => {
//             console.log("onSuccess")
//             // @ts-ignore
//             // refetch auth data...
//         },
//         onSettled: () => {
//         },
//     })
//     return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
// }

