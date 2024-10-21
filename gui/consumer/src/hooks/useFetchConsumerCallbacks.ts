import { transferCallbackModelFromDTO } from "@/lib/mappers";
import { useQuery } from "@tanstack/react-query";

export const useFetchConsumerCallbacks = () => {
  console.log(import.meta.env.VITE_SOME_KEY); // "123"

  const { data, isError, error } = useQuery({
    queryKey: [],
    queryFn: async () => {
      const consumerCallbacks = await fetch(
        "http://localhost:1235/api/v1/callbacks"
      );
      return transferCallbackModelFromDTO(await consumerCallbacks.json());
    },
    refetchInterval: 10000,
    refetchIntervalInBackground: true,
  });
  return { data, isError, error };
};
