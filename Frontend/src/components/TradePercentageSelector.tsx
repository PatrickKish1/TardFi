"use client";
import { useState } from "react";
import { Button } from "./ui/button";

const percentages = [10, 25, 50, 75, 100];

export interface TradePercentageSelectorProps {
  onSelect: (percent: number) => void;
}

const TradePercentageSelector = ({ onSelect }: TradePercentageSelectorProps) => {
  const [selected, setSelected] = useState<number | null>(null);

  const handleSelect = (percent: number) => {
    setSelected(percent);
    onSelect(percent);
  };

  return (
    <div className="w-full">
      <div className="flex justify-between gap-2 mt-4">
        {percentages.map((percent) => (
          <Button
            key={percent}
            onClick={() => handleSelect(percent)}
            variant={selected === percent ? "default" : "outline"}
            className={
              selected === percent
                ? "bg-yellow-400 text-black border-yellow-400"
                : "bg-gray-800 text-white border-gray-600"
            }
          >
            {percent}%
          </Button>
        ))}
      </div>
    </div>
  );
};

export default TradePercentageSelector; 