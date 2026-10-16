import React from "react";
import { cn } from "shared/src/lib/utils";
import { useParticipantDirectory } from "shared/src/data/useParticipantDirectory";
import { Identifier } from "./Identifier";

export interface ParticipantLabelProps {
  id?: string | null;
  className?: string;
  /** Hides the identifier line under the name. */
  compact?: boolean;
}

/** A participant by its nickname, marked when it is this agent, with its DID underneath. */
export function ParticipantLabel({ id, className, compact = false }: ParticipantLabelProps) {
  const { byId, isSelf } = useParticipantDirectory();
  if (!id) return <span className="text-ink/35">—</span>;

  const nick = byId.get(id)?.participant_nick;
  const self = isSelf(id);

  return (
    <span className={cn("inline-flex min-w-0 flex-col", className)}>
      <span className="inline-flex items-center gap-1.5">
        <span className={cn("truncate font-medium", !nick && "text-ink/60")}>
          {nick ?? "Unknown participant"}
        </span>
        {self && (
          <span className="rounded bg-brand-sky/15 px-1.5 py-px text-[0.6875rem] font-semibold text-brand-sky">
            You
          </span>
        )}
      </span>
      {!compact && <Identifier value={id} />}
    </span>
  );
}
