import * as React from "react";
import { ChevronDownIcon } from "lucide-react";

import { Button } from "@/shared/ui/button";
import { Calendar } from "@/shared/ui/calendar";
import { Popover, PopoverContent, PopoverTrigger } from "@/shared/ui/popover";
import { Label } from "./label";

export type DatePickerProps = {
  initialValue?: Date;
  onDatePicked: (date?: Date) => void;
  label: string;
};

export function DatePicker({ initialValue, onDatePicked, label }: DatePickerProps) {
  const [date, setDate] = React.useState<Date | undefined>(initialValue);
  const [open, setOpen] = React.useState(false);

  React.useEffect(() => {
    onDatePicked(date);
  }, [date, onDatePicked]);

  return (
    <>
      <Label htmlFor={`date-${label}`} className="px-1">
        {label}
      </Label>
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button
            variant="outline"
            id={`date-${label}`}
            className="w-48 justify-between font-normal"
          >
            {date ? date.toLocaleDateString() : "Select date"}
            <ChevronDownIcon />
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-auto overflow-hidden p-0" align="start">
          <Calendar
            mode="single"
            selected={date}
            captionLayout="dropdown"
            onSelect={(date) => {
              setDate(date);
              setOpen(false);
            }}
          />
        </PopoverContent>
      </Popover>
    </>
  );
}
