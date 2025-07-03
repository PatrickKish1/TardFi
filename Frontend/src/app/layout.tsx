import type { Metadata } from "next";
import { Geist, Geist_Mono, Montserrat } from "next/font/google";

import "./globals.css";
import "@rainbow-me/rainbowkit/styles.css";
import { Navbar, Footer } from "../components";
import Providers from "./providers";
import { headers } from "next/headers";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

const montserrat = Montserrat({
  subsets: ['latin'], // Always specify subsets to reduce font file size
  display: 'swap',   // Ensures text remains visible during font loading
  variable: '--font-montserrat', // Define a CSS variable for Tailwind to use
  // You can also specify specific weights if you only need a few:
  // weight: ['400', '700'],
  // Or even styles if you need italics:
  // style: ['normal', 'italic'],
});

export const metadata: Metadata = {
  title: "TardFI",
  description: "Where Oil meets blockchain",
};

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  const cookie = (await headers()).get("cookie");
  return (
    <html lang="en">
      <body className={`${montserrat.variable} antialiased font-montserrat`}>
        <Providers>
          <Navbar />
          <main className="flex-1 w-full">{children}</main>
          <Footer />
        </Providers>
      </body>
    </html>
  );
}
