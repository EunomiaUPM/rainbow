import React, { FC } from "react";
import { Button, ButtonSizes } from "shared/src/components/ui/button";
import { Dialog, DialogTrigger } from "shared/src/components/ui/dialog";

interface ProcessActionDialogProps {
  label: string;
  variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link" | "icon_destructive";
  tiny?: boolean;
  DialogComponent: FC<{ process: any }>; // Generic process type
  process: any;
}

/**
 * Reusable dialog component for process actions.
 */
export const ProcessActionDialog: FC<ProcessActionDialogProps> = ({
  label,
  variant,
  tiny,
  DialogComponent,
  process,
}) => {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant={variant} size={(tiny ? "sm" : "") as ButtonSizes}>
          {label}
        </Button>
      </DialogTrigger>
      <DialogComponent process={process} />
    </Dialog>
  );
};
