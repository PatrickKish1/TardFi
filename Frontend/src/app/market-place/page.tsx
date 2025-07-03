import { OilCard } from "../../components";
import { marketData } from "../../lib/mockData";

export default function MarketPlace() {
  return (
    <div className="w-full pt-[4rem] text-white">
      <div className="layout">
        <h2 className="text-xl font-semibold md:text-2xl">Marketplace</h2>
        <div className="mt-4 w-full flex items-start md:items-center gap-6 md:gap-0 flex-col md:flex-row justify-between">
          <div>
            <input
              type="text"
              placeholder="search for oil type sellers, etc"
              className="w-[280px] md:w-[500px] rounded-2xl border border-gray-800 text-sm px-4 h-9 outline-0"
            />
          </div>
          <div className="flex items-center gap-x-2 md:gap-x-6">
            <h3 className="border border-gray-800 px-3 py-1.5 md:py-1 text-xs md:text-base rounded-xl">All</h3>
            <h3 className="border border-gray-800 px-3 py-1.5 md:py-1 text-xs md:text-base rounded-xl">Crude oil</h3>
            <h3 className="border border-gray-800 px-3 py-1.5 md:py-1 text-xs md:text-base rounded-xl">Refined oil</h3>
            <h3 className="border border-gray-800 px-3 py-1.5 md:py-1 text-xs md:text-base rounded-xl">Synthetic oil</h3>
          </div>
        </div>
        <div className="mt-12 grid grid-cols-1 md:grid-cols-2 gap-7 lg:grid-cols-4">
          {marketData.map((item) => (
            <OilCard key={item.id} {...item} />
          ))}
        </div>
      </div>
    </div>
  );
} 