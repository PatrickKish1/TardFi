import Image from "next/image";
import { features, platformFeatures } from "../lib/mockData";
import { Button } from "../components/ui/button";
import { ConnectButton } from "@rainbow-me/rainbowkit";

export default function Home() {
  return (
    <div className="text-white w-full pt-[3rem]">
      <div className="layout">
        <div className="w-full flex items-start justify-between pt-4 md:pt-7 flex-col md:flex-row gap-10">
          <div className="w-full">
            <h2 className="text-2xl md:text-3xl text-gradient lg:text-4xl font-bold leading-[3rem] md:leading-[5rem]">
              Revolutionize Oil Trading <br /> with AI & Web3
            </h2>
            <p className="text-sm md:text-base lg:text-lg py-6">
              Trade oil smarter, faster, and more securely. Experience real-time
              market insights, AI-driven decisions, and a decentralized
              marketplace all in one sleek platform.
            </p>
            <div className="mt-4 flex items-center gap-8">
              <div className="custom-gradient rounded-2xl px-4 py-2 text-white">
                <ConnectButton showBalance={false} chainStatus="icon" />
              </div>
              <Button variant="outline" size={'lg'} className="border text-black border-[#dadada] px-24 py-6 text-xl rounded-2xl">
                Explore
              </Button>
            </div>
          </div>
          <div className="w-full">
            <div className="w-[90%] mx-auto">
              <Image src="/drum.png" alt="Drum" width={600} height={400} className="w-full" />
            </div>
          </div>
        </div>
        <section className="my-8">
          <h2 className="text-2xl text-gradient font-semibold md:text-3xl capitalize">
            Feature marketplace
          </h2>
          <div className="w-full mt-6 mx-auto grid  grid-cols-2 gap-5 md:gap-10 md:grid-cols-4">
            {features.map((item) => (
              <div className="w-[90%] mx-auto md:w-[95%]" key={item.id}>
                <Image
                  src={item.icon}
                  alt={item.title}
                  width={300}
                  height={140}
                  className="w-full rounded-xl h-[140px] object-cover"
                />
                <h3 className="text-base font-semibold py-1">{item.title}</h3>
                <p className="text-xs md:text-sm">{item.desc}</p>
              </div>
            ))}
          </div>
        </section>
        <section className="mt-16 mb-3 ">
          <h2 className="text-2xl text-gradient font-semibold md:text-3xl capitalize">
            TardFi Features
          </h2>
          <div className="mt-7 grid  grid-cols-1 md:grid-cols-2 gap-7 md:gap-8 lg:grid-cols-4">
            {platformFeatures.map((item) => {
              const Icon = item.icon;
              return (
                <div
                  className="border border-[#dadada] rounded-2xl py-4 w-[90%] mx-auto md:w-full"
                  key={item.id}
                >
                  <div className="w-[90%]  mx-auto">
                    <div className="text-white pb-3">
                      <Icon className="size-6 md:size-8" />
                    </div>
                    <h3 className="text-lg md:text-xl font-semibold py-2 ">
                      {item.title}
                    </h3>
                    <p className="text-sm">{item.desc}</p>
                  </div>
                </div>
              );
            })}
          </div>
        </section>
      </div>
    </div>
  );
}
