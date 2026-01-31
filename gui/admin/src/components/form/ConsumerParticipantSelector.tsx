import { Control } from "react-hook-form";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "shared/src/components/ui/form";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "shared/src/components/ui/popover";
import { Button } from "shared/src/components/ui/button";
import { ChevronsUpDown } from "lucide-react";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "shared/src/components/ui/command";

interface ConsumerParticipantSelectorProps {
  control: Control<any>;
  name: string;
  participants: Participant[];
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onSelect: (participant: Participant) => void;
  isLoading?: boolean;
}

export const ConsumerParticipantSelector = ({
  control,
  name,
  participants,
  isOpen,
  onOpenChange,
  onSelect,
}: ConsumerParticipantSelectorProps) => {
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormLabel>Consumer Participant Id</FormLabel>
          <div>
            <FormControl>
              <Popover open={isOpen} onOpenChange={onOpenChange}>
                <PopoverTrigger asChild>
                  <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={isOpen}
                    className="w-full justify-between font-normal text-gray-300 transition-colors"
                  >
                    {field.value
                      ? participants.find((p) => p.participant_id === field.value)
                        ?.participant_id
                      : "Select participant..."}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-80" />
                  </Button>
                </PopoverTrigger>
                <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                  <Command>
                    <CommandInput placeholder="Search participant..." />
                    <CommandList>
                      <CommandEmpty>No participant found.</CommandEmpty>
                      <CommandGroup>
                        {participants.map((consumerParticipant) => (
                          <CommandItem
                            key={consumerParticipant.participant_id}
                            value={consumerParticipant.participant_id}
                            onSelect={() => {
                              field.onChange(consumerParticipant.participant_id);
                              onSelect(consumerParticipant);
                            }}
                            className={
                              field.value === consumerParticipant.participant_id
                                ? "text-role-consumer font-medium"
                                : ""
                            }
                          >
                            {consumerParticipant.participant_id}
                            <span className="text-gray-400 ml-2 text-sm">
                              ({consumerParticipant.base_url})
                            </span>
                          </CommandItem>
                        ))}
                      </CommandGroup>
                    </CommandList>
                  </Command>
                </PopoverContent>
              </Popover>
            </FormControl>
            <FormDescription className="text-sm text-gray-400 mt-1">
              Provide the ID of the consumer participant for the negotiation.
            </FormDescription>
            <FormMessage />
          </div>
        </FormItem>
      )}
    />
  );
};
