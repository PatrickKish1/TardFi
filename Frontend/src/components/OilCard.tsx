"use client";
import { useRouter } from "next/navigation";
import Image from "next/image";
import { Card } from "./ui/card";
import { Button } from "./ui/button";

export interface OilCardProps {
  id: number;
  location: string;
  seller: string;
  quantity: string;
  icon: string;
}

const OilCard = ({ id, location, seller, quantity, icon }: OilCardProps) => {
  const router = useRouter();
  return (
    <Card className="w-[90%] mx-auto md:w-full border border-gray-800 shadow rounded-2xl">
      <Image src={icon} alt={location} width={400} height={176} className="w-full rounded-2xl h-44 object-cover" />
      <div className="w-[94%] mx-auto py-2">
        <h2 className="text-base font-semibold">{location}</h2>
        <div className="flex items-center gap-x-1.5 text-sm py-2">
          <p>Seller: </p>
          <p>{seller}</p>
        </div>
        <div className="flex items-center gap-x-1.5 text-sm">
          <p>Quantity: </p>
          <p>{quantity} barrels</p>
        </div>
        <div className="w-full pt-5 pb-2">
          <Button
            className="bg-green-700 cursor-pointer capitalize text-sm w-full rounded-2xl py-1"
            onClick={() => router.push(`/trade/${id}`)}
          >
            Buy now
          </Button>
        </div>
      </div>
    </Card>
  );
};

export default OilCard; 