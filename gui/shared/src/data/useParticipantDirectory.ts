import { useMemo } from "react";
import { useGetAllParticipants } from "shared/src/data/orval/participants/participants";
import { ParticipantDto } from "shared/src/data/orval/model";
import { useMyself } from "shared/src/data/useMyself";

export interface ParticipantDirectory {
  self?: ParticipantDto;
  byId: Map<string, ParticipantDto>;
  isSelf: (id?: string | null) => boolean;
}

/** Known participants by id plus this agent itself, cached across the app by react-query. */
export const useParticipantDirectory = (): ParticipantDirectory => {
  const self = useMyself();
  const { data } = useGetAllParticipants({ limit: 200 }, { query: { staleTime: 5 * 60_000 } });

  return useMemo(() => {
    const byId = new Map<string, ParticipantDto>();
    const items = data?.status === 200 ? data.data.items : [];
    for (const p of items) byId.set(p.participant_id, p);
    if (self) byId.set(self.participant_id, self);
    return {
      self,
      byId,
      isSelf: (id) => !!id && !!self && id === self.participant_id,
    };
  }, [data, self]);
};
