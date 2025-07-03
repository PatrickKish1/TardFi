"use client";
import { Chart, TradePercentageSelector } from "../../../components";
import { useState } from "react";

export default function Trade() {
  const [selected, setSelected] = useState<number | null>(null);
  return (
    <div className="w-full pt-[4rem] text-white">
      <div className="layout">
        <div className="bg-gray-800 w-full h-10 "></div>
        <div className="w-full flex items-start flex-col md:flex-row gap-4 mt-6">
          <div className="w-full md:w-[70%] bg-gray-900">
            <div className="w-[96%] py-1.5 mx-auto">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-x-2 md:gap-x-4">
                  <h3 className="text-xs md:text-sm ">Chart</h3>
                  <h3 className="text-xs md:text-sm ">Info</h3>
                  <h3 className="text-xs md:text-sm ">TradingData</h3>
                </div>
                <div className="hidden md:flex items-center gap-x-3">
                  <h4 className="text-xs md:text-sm ">Standard</h4>
                  <h4 className="text-xs md:text-sm ">Tradingview</h4>
                  <h4 className="text-xs md:text-sm ">Depth</h4>
                </div>
              </div>
              <div className="mt-2">
                <Chart />
              </div>
            </div>
          </div>
          <div className="w-full md:w-[30%] bg-gray-900 rounded-lg">
            <div className="w-[90%] mx-auto py-2">
              <h2 className="text-base  md:text-lg font-semibold mb-2">
                Buy Oil
              </h2>
              <div className="flex items-center gap-x-3">
                <h3 className="text-sm">Limit</h3>
                <h3 className="text-sm">Market</h3>
                <h3 className="text-sm">Conditional</h3>
              </div>
              <div className="mt-4 ">
                <h4 className="text-sm">Order by value</h4>
                <div className="bg-black h-8 w-full mt-3.5 rounded-lg ">
                  <input type="text" className="w-full outline-0 px-2" />
                </div>
              </div>
              <TradePercentageSelector onSelect={setSelected} />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
} 