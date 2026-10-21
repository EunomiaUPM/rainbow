import React, { useMemo, useState } from "react";
import { PrismLight as SyntaxHighlighter } from "react-syntax-highlighter";
import json from "react-syntax-highlighter/dist/esm/languages/prism/json";
import oneDark from "react-syntax-highlighter/dist/esm/styles/prism/one-dark";
import oneLight from "react-syntax-highlighter/dist/esm/styles/prism/one-light";
import { Check, Copy } from "lucide-react";
import { cn } from "shared/src/lib/utils";
import { useTheme } from "shared/src/hooks/useTheme";

SyntaxHighlighter.registerLanguage("json", json);

export interface JsonViewerProps {
  value: unknown;
  className?: string;
  /** Tailwind max-height class for the scroll area. */
  maxHeightClass?: string;
}

/** Pretty-printed, highlighted JSON with a copy button. */
export function JsonViewer({ value, className, maxHeightClass = "max-h-[60vh]" }: JsonViewerProps) {
  const { resolvedTheme } = useTheme();
  const [copied, setCopied] = useState(false);
  const text = useMemo(
    () => (typeof value === "string" ? JsonViewer.prettify(value) : JSON.stringify(value, null, 2)),
    [value],
  );

  const copy = () => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  return (
    <div className={cn("group relative rounded-lg border border-ink/10 bg-sunken", className)}>
      <button
        type="button"
        onClick={copy}
        aria-label="Copy JSON"
        className="absolute right-2 top-2 z-10 inline-flex items-center gap-1 rounded-md border border-ink/10 bg-background/80 px-2 py-1 text-xs text-ink/70 opacity-0 backdrop-blur transition-opacity hover:text-ink group-hover:opacity-100 focus-visible:opacity-100"
      >
        {copied ? <Check className="h-3.5 w-3.5" /> : <Copy className="h-3.5 w-3.5" />}
        {copied ? "Copied" : "Copy"}
      </button>
      <div className={cn("overflow-auto", maxHeightClass)}>
        <SyntaxHighlighter
          language="json"
          style={resolvedTheme === "dark" ? oneDark : oneLight}
          customStyle={{
            margin: 0,
            padding: "0.875rem 1rem",
            background: "transparent",
            fontSize: "0.8125rem",
            lineHeight: 1.6,
          }}
          codeTagProps={{
            style: { fontFamily: '"Source Code Pro", monospace', background: "transparent" },
          }}
          wrapLongLines
        >
          {text}
        </SyntaxHighlighter>
      </div>
    </div>
  );
}

/** Re-indents a JSON string, leaving non-JSON text untouched. */
JsonViewer.prettify = (raw: string): string => {
  try {
    return JSON.stringify(JSON.parse(raw), null, 2);
  } catch {
    return raw;
  }
};
