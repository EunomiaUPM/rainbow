import { queryOptions, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../context/GlobalInfoContext";

/**
 *  GET /connector/instances/{id}
 * */
export const getConnectorInstanceById = async (api_gateway: string, id: string) => {
  const instances: Catalog[] = await (
    await fetch(api_gateway + "/connector/instances/" + id)
  ).json();
  return instances;
};

export const getConnectorInstanceByIdOptions = (api_gateway: string, id: string) =>
  queryOptions({
    queryKey: ["CONNECTOR_INSTANCE_BY_ID", id],
    queryFn: () => getConnectorInstanceById(api_gateway, id),
  });

export const useGetConnectorInstancesById = (id: string) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getConnectorInstanceByIdOptions(api_gateway, id),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /connector/instances/distribution/{distribution_id}
 * */
export const getConnectorInstanceByDistribution = async (api_gateway: string, id: string) => {
  const instances: ConnectorInstanceDto = await (
    await fetch(api_gateway + "/connector/instances/distribution/" + id)
  ).json();
  return instances;
};

export const getConnectorInstanceByDistributionOptions = (api_gateway: string, id: string) =>
  queryOptions({
    queryKey: ["CONNECTOR_INSTANCE_BY_DISTRIBUTION", id],
    queryFn: () => getConnectorInstanceByDistribution(api_gateway, id),
  });

export const useGetConnectorInstancesByDistribution = (id: string) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getConnectorInstanceByDistributionOptions(api_gateway, id),
  );
  return { data, isLoading, isError, error };
};
