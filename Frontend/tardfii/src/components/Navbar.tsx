"use client";
import { useState } from "react";
import Link from "next/link";
import { Button } from "./ui/button";
import { IoMenuOutline } from "react-icons/io5";
import { IoMdClose } from "react-icons/io";
import { ConnectButton } from "@rainbow-me/rainbowkit";

const Navbar = () => {
  const [toggle, setToggle] = useState(false);
  return (
    <div className="w-full fixed h-12 md:h-16 flex shadow items-center bg-[#030710] z-50">
      <div className="layout flex items-center justify-between">
        <Link href="/" className="text-white text-xl sm:text-2xl lg:text-3xl">
          TardFi
        </Link>
        <div className="hidden text-white md:flex items-center gap-x-6 bg-gray-900 rounded-3xl py-2 px-8">
          <Link href="/">Home</Link>
          <Link href="/market-place">Marketplace</Link>
          <Link href="/">Buy</Link>
          <Link href="/">Sell</Link>
        </div>
        <div className="hidden md:flex items-center md:gap-x-8 lg:gap-x-12">
          <ConnectButton showBalance={false} chainStatus="icon" />
        </div>
        <div className="block md:hidden text-white cursor-pointer" onClick={() => setToggle(!toggle)}>
          {toggle ? <IoMdClose className="size-8" /> : <IoMenuOutline className="size-8" />}
        </div>
      </div>
      {/* Mobile menu */}
      {toggle && (
        <div className="absolute top-12 left-0 w-full bg-[#030710] shadow-md md:hidden z-50">
          <div className="flex flex-col items-center gap-y-4 py-4">
            <Link href="/" className="text-white text-lg" onClick={() => setToggle(false)}>
              Home
            </Link>
            <Link href="/market-place" className="text-white text-lg" onClick={() => setToggle(false)}>
              Marketplace
            </Link>
            <Link href="/" className="text-white text-lg" onClick={() => setToggle(false)}>
              Buy
            </Link>
            <Link href="/" className="text-white text-lg" onClick={() => setToggle(false)}>
              Sell
            </Link>
            <ConnectButton showBalance={false} chainStatus="icon" />
          </div>
        </div>
      )}
    </div>
  );
};

export default Navbar; 