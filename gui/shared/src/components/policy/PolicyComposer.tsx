import React, { useMemo, useState } from "react";
import { Ban, CheckCircle2, ClipboardList, Plus, Trash2, X } from "lucide-react";
import { cn } from "shared/src/lib/utils";
import { Button } from "shared/src/components/ui/button";
import { Input } from "shared/src/components/ui/input";
import { Textarea } from "shared/src/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "shared/src/components/ui/dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "shared/src/components/ui/tabs";
import { JsonViewer } from "shared/src/components/json/JsonViewer";
import { OdrlRulesView } from "./OdrlRulesView";
import { leftOperands, odrlActions, operators } from "shared/src/odrl_actions";
import { formatOperator } from "shared/src/components/ui/format-operator";
import { OdrlPolicyInfo } from "shared/src/data/orval/model/odrlPolicyInfo";

type Kind = "permission" | "prohibition" | "obligation";
interface Constraint {
  leftOperand: string;
  operator: string;
  rightOperand: string;
}
interface Rule {
  kind: Kind;
  action: string;
  constraints: Constraint[];
}

const KINDS: {
  kind: Kind;
  label: string;
  Icon: React.ComponentType<{ className?: string }>;
  tone: string;
}[] = [
  {
    kind: "permission",
    label: "Permission",
    Icon: CheckCircle2,
    tone: "text-success-600 dark:text-success-400",
  },
  {
    kind: "prohibition",
    label: "Prohibition",
    Icon: Ban,
    tone: "text-danger-600 dark:text-danger-400",
  },
  {
    kind: "obligation",
    label: "Obligation",
    Icon: ClipboardList,
    tone: "text-warn-600 dark:text-warn-400",
  },
];

/** Common starting points; each is plain ODRL the user can edit afterwards. */
const PRESETS: { label: string; description: string; rules: Rule[] }[] = [
  {
    label: "Open use",
    description: "Anyone with an agreement may use the data.",
    rules: [{ kind: "permission", action: "use", constraints: [] }],
  },
  {
    label: "Research only",
    description: "Use for research; no redistribution.",
    rules: [
      {
        kind: "permission",
        action: "use",
        constraints: [{ leftOperand: "purpose", operator: "eq", rightOperand: "research" }],
      },
      { kind: "prohibition", action: "distribute", constraints: [] },
    ],
  },
  {
    label: "Time limited",
    description: "Use until a given date.",
    rules: [
      {
        kind: "permission",
        action: "use",
        constraints: [
          { leftOperand: "dateTime", operator: "lt", rightOperand: "2026-12-31T23:59:59Z" },
        ],
      },
    ],
  },
];

export interface PolicyComposerProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  /** What the policy is attached to, shown in the title. */
  targetLabel?: React.ReactNode;
  onSubmit: (policy: OdrlPolicyInfo, description?: string) => Promise<void> | void;
}

/** Dialog to write an ODRL offer rule by rule, with a live preview of the result. */
export function PolicyComposer({ open, onOpenChange, targetLabel, onSubmit }: PolicyComposerProps) {
  const [rules, setRules] = useState<Rule[]>([]);
  const [description, setDescription] = useState("");
  const [submitting, setSubmitting] = useState(false);

  const policy = useMemo(() => PolicyComposer.toOdrl(rules), [rules]);
  const problem =
    rules.length === 0
      ? "Add at least one rule."
      : rules.some((r) => !r.action)
        ? "Every rule needs an action."
        : rules.some((r) => r.constraints.some((c) => !c.leftOperand || !c.rightOperand))
          ? "Complete or remove the empty constraints."
          : undefined;

  const update = (i: number, change: Partial<Rule>) =>
    setRules((rs) => rs.map((r, j) => (j === i ? { ...r, ...change } : r)));
  const updateConstraint = (i: number, k: number, change: Partial<Constraint>) =>
    update(i, {
      constraints: rules[i].constraints.map((c, j) => (j === k ? { ...c, ...change } : c)),
    });

  const reset = () => {
    setRules([]);
    setDescription("");
  };
  const submit = async () => {
    setSubmitting(true);
    try {
      await onSubmit(policy, description.trim() || undefined);
      reset();
      onOpenChange(false);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[90vh] max-w-5xl flex-col gap-0 p-0">
        <DialogHeader className="border-b border-ink/10 px-6 py-4">
          <DialogTitle>New access policy</DialogTitle>
          {targetLabel && <DialogDescription>For {targetLabel}</DialogDescription>}
        </DialogHeader>

        <div className="grid min-h-0 flex-1 grid-cols-1 overflow-hidden lg:grid-cols-[1fr_minmax(0,0.85fr)]">
          <div className="space-y-5 overflow-y-auto px-6 py-5">
            <div className="space-y-1.5">
              <label className="text-sm font-medium">Description</label>
              <Textarea
                rows={2}
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="What this offer allows, in a sentence consumers will read in your catalog"
                className="resize-none"
              />
            </div>

            <div className="space-y-2">
              <span className="text-sm font-medium">Start from</span>
              <div className="grid grid-cols-1 gap-2 sm:grid-cols-3">
                {PRESETS.map((p) => (
                  <button
                    key={p.label}
                    type="button"
                    onClick={() => setRules(structuredClone(p.rules))}
                    className="rounded-lg border border-ink/10 p-3 text-left transition-colors hover:border-brand-sky/50 hover:bg-ink/[0.03]"
                  >
                    <span className="block text-sm font-medium">{p.label}</span>
                    <span className="mt-0.5 block text-xs text-ink/55">{p.description}</span>
                  </button>
                ))}
              </div>
            </div>

            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium">Rules</span>
                <div className="flex gap-1.5">
                  {KINDS.map(({ kind, label, Icon, tone }) => (
                    <Button
                      key={kind}
                      size="xs"
                      variant="outline"
                      onClick={() =>
                        setRules((rs) => [...rs, { kind, action: "", constraints: [] }])
                      }
                    >
                      <Icon className={tone} />
                      {label}
                    </Button>
                  ))}
                </div>
              </div>

              {rules.length === 0 && (
                <p className="rounded-lg border border-dashed border-ink/15 py-6 text-center text-sm text-ink/50">
                  Pick a starting point or add a rule.
                </p>
              )}

              {rules.map((rule, i) => {
                const meta = KINDS.find((k) => k.kind === rule.kind)!;
                return (
                  <div key={i} className="rounded-lg border border-ink/10 bg-ink/[0.015] p-3">
                    <div className="flex items-center gap-2">
                      <meta.Icon className={cn("h-4 w-4 shrink-0", meta.tone)} />
                      <span className="w-24 text-sm font-medium">{meta.label}</span>
                      <Select value={rule.action} onValueChange={(action) => update(i, { action })}>
                        <SelectTrigger className="h-8 w-44">
                          <SelectValue placeholder="Action" />
                        </SelectTrigger>
                        <SelectContent>
                          {odrlActions.map((a) => (
                            <SelectItem key={a} value={a}>
                              {a}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                      <div className="ml-auto flex items-center gap-1">
                        <Button
                          size="xs"
                          variant="ghost"
                          onClick={() =>
                            update(i, {
                              constraints: [
                                ...rule.constraints,
                                { leftOperand: "", operator: "eq", rightOperand: "" },
                              ],
                            })
                          }
                        >
                          <Plus /> Constraint
                        </Button>
                        <Button
                          size="icon_sm"
                          variant="ghost"
                          aria-label="Remove rule"
                          onClick={() => setRules((rs) => rs.filter((_, j) => j !== i))}
                        >
                          <Trash2 />
                        </Button>
                      </div>
                    </div>

                    {rule.constraints.length > 0 && (
                      <div className="mt-3 space-y-2 border-l-2 border-ink/10 pl-3">
                        {rule.constraints.map((c, k) => (
                          <div key={k} className="flex items-center gap-2">
                            <Select
                              value={c.leftOperand}
                              onValueChange={(leftOperand) =>
                                updateConstraint(i, k, { leftOperand })
                              }
                            >
                              <SelectTrigger className="h-8 w-36">
                                <SelectValue placeholder="Operand" />
                              </SelectTrigger>
                              <SelectContent>
                                {leftOperands.map((o) => (
                                  <SelectItem key={o} value={o}>
                                    {o}
                                  </SelectItem>
                                ))}
                              </SelectContent>
                            </Select>
                            <Select
                              value={c.operator}
                              onValueChange={(operator) => updateConstraint(i, k, { operator })}
                            >
                              <SelectTrigger className="h-8 w-40">
                                <SelectValue />
                              </SelectTrigger>
                              <SelectContent>
                                {operators.map((o) => (
                                  <SelectItem key={o} value={o}>
                                    {formatOperator(o)}
                                  </SelectItem>
                                ))}
                              </SelectContent>
                            </Select>
                            <Input
                              className="h-8 flex-1"
                              value={c.rightOperand}
                              placeholder="Value"
                              onChange={(e) =>
                                updateConstraint(i, k, { rightOperand: e.target.value })
                              }
                            />
                            <Button
                              size="icon_sm"
                              variant="ghost"
                              aria-label="Remove constraint"
                              onClick={() =>
                                update(i, {
                                  constraints: rule.constraints.filter((_, j) => j !== k),
                                })
                              }
                            >
                              <X />
                            </Button>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          </div>

          <div className="overflow-y-auto border-t border-ink/10 bg-ink/[0.015] px-6 py-5 lg:border-l lg:border-t-0">
            <Tabs defaultValue="rules">
              <TabsList>
                <TabsTrigger value="rules">Preview</TabsTrigger>
                <TabsTrigger value="json">ODRL</TabsTrigger>
              </TabsList>
              <TabsContent value="rules" className="pt-4">
                <OdrlRulesView policy={policy} />
              </TabsContent>
              <TabsContent value="json" className="pt-4">
                <JsonViewer value={policy} />
              </TabsContent>
            </Tabs>
          </div>
        </div>

        <DialogFooter className="flex items-center gap-3 border-t border-ink/10 px-6 py-3 sm:justify-between">
          <span className="text-sm text-ink/55">{problem}</span>
          <div className="flex gap-2">
            <Button variant="outline" onClick={() => onOpenChange(false)}>
              Cancel
            </Button>
            <Button onClick={submit} disabled={!!problem} isLoading={submitting}>
              Add policy
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/** Rules grouped as ODRL; empty groups are left out. */
PolicyComposer.toOdrl = (rules: Rule[]): OdrlPolicyInfo => {
  const group = (kind: Kind) =>
    rules
      .filter((r) => r.kind === kind)
      .map((r) => ({
        action: r.action,
        ...(r.constraints.length > 0 && { constraint: r.constraints }),
      }));
  const out: Record<string, unknown> = {};
  for (const { kind } of KINDS) {
    const g = group(kind);
    if (g.length > 0) out[kind] = g;
  }
  return out as OdrlPolicyInfo;
};
