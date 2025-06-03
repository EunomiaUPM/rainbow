import {GATEWAY_API} from "@/data/index.ts";
import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";

/**
 *  GET /datasets/{datasetId}/policies
 * */
export const getPoliciesByDatasetId = async (datasetId: UUID) => {
    const catalog: OdrlOffer[] = await (
        await fetch(GATEWAY_API + `/datasets/${datasetId}/policies`)
    ).json();
    return catalog;
}

export const getPoliciesByDatasetIdOptions = (datasetId: UUID) => queryOptions({
    queryKey: ["POLICIES_BY_DATASET_ID", datasetId],
    queryFn: () => getPoliciesByDatasetId(datasetId),
    enabled: !!datasetId
})

export const useGetPoliciesByDatasetId = (datasetId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getPoliciesByDatasetIdOptions(datasetId))
    return {data, isLoading, isError, error}
}