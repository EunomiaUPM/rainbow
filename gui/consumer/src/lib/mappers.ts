export const transferCallbackModelFromDTO = (
  transferCallbackModelDTOs: Array<TransferCallbackModelDTO>
): Array<TransferCallbackModel> => {
  return transferCallbackModelDTOs.map((transferCallbackModelDTO) => ({
    id: transferCallbackModelDTO.id,
    created_at: new Date(transferCallbackModelDTO.created_at),
    updated_at: new Date(transferCallbackModelDTO.created_at),
    consumer_pid: transferCallbackModelDTO.consumer_pid,
    provider_pid: transferCallbackModelDTO.provider_pid,
    data_address: transferCallbackModelDTO.data_address,
  }));
};
