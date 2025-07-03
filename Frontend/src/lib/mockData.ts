// Features for the marketplace
export interface Feature {
  id: number;
  title: string;
  desc: string;
  icon: string; // public path to image
}

export const features: Feature[] = [
  {
    id: 1,
    title: "Brent crude oil",
    desc: "The global benchmark for crude oil",
    icon: "/chart.png",
  },
  {
    id: 2,
    title: "Brent crude oil",
    desc: "The global benchmark for crude oil",
    icon: "/oil.jpg",
  },
  {
    id: 3,
    title: "Brent crude oil",
    desc: "The global benchmark for crude oil",
    icon: "/icon.jpeg",
  },
  {
    id: 4,
    title: "Brent crude oil",
    desc: "The global benchmark for crude oil",
    icon: "/chart.png",
  },
];

// Platform features
import { FaArrowRightArrowLeft, FaLock } from "react-icons/fa6";
import { FaBrain } from "react-icons/fa";
import { MdOutlineShield } from "react-icons/md";

export interface PlatformFeature {
  id: number;
  icon: React.ElementType;
  title: string;
  desc: string;
}

export const platformFeatures: PlatformFeature[] = [
  {
    id: 1,
    icon: FaArrowRightArrowLeft,
    title: "Decentralized Trading ",
    desc: "Trade directly with other users without intermediaries, ensuring greater control and efficiency.",
  },
  {
    id: 2,
    icon: FaBrain,
    title: "AI forecasting ",
    desc: "Leverage advanced AI algorithm for predictive market analysis and personalized trading strategies.",
  },
  {
    id: 3,
    icon: MdOutlineShield,
    title: "Secure wallet ",
    desc: "Safeguard your assets with our integrated secure wallet, designed for web3 security.",
  },
  {
    id: 4,
    icon: FaLock,
    title: "Transparent transaction ",
    desc: "Benefit from the transparency of blockchain technology.",
  },
];

// Market data
export interface MarketData {
  id: number;
  location: string;
  seller: string;
  quantity: string;
  icon: string; // public path to image
}

export const marketData: MarketData[] = [
  { id: 1, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "3000", icon: "/oil.jpg" },
  { id: 2, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "3000", icon: "/chart.png" },
  { id: 3, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "3000", icon: "/icon.jpeg" },
  { id: 4, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "3000", icon: "/oil.jpg" },
  { id: 5, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "3000", icon: "/chart.png" },
  { id: 6, location: "Nigeria Oil & gas", seller: "Global Energy Ltd.", quantity: "3400", icon: "/oil.jpg" },
  { id: 7, location: "Brent Crude", seller: "Global Energy Ltd.", quantity: "3000", icon: "/icon.jpeg" },
  { id: 8, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "3000", icon: "/chart.png" },
  { id: 9, location: "OPEC basket", seller: "Global Energy Ltd.", quantity: "3000", icon: "/oil.jpg" },
  { id: 10, location: "Brent Crude", seller: "Global Energy Ltd.", quantity: "3000", icon: "/icon.jpeg" },
  { id: 11, location: "Dubai Crude", seller: "Global Energy Ltd.", quantity: "3000", icon: "/oil.jpg" },
  { id: 12, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "200", icon: "/chart.png" },
  { id: 13, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "1500", icon: "/icon.jpeg" },
  { id: 14, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "500", icon: "/oil.jpg" },
  { id: 15, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "3900", icon: "/chart.png" },
  { id: 16, location: "West Texas intermediate", seller: "Global Energy Ltd.", quantity: "1000", icon: "/oil.jpg" },
]; 