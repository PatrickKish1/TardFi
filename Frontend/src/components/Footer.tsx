import Link from "next/link";
import { FaTwitter, FaGithub } from "react-icons/fa";

const Footer = () => {
  return (
    <footer className="text-white w-full pt-5 pb-4 bg-[#030710] border-t border-gray-800 mt-8">
      <div className="layout flex flex-col md:flex-row items-center justify-between gap-4">
        <div className="flex items-center gap-2">
          <span className="font-bold text-lg">TardFi</span>
          <span className="text-xs text-gray-400">&copy; {new Date().getFullYear()} All rights reserved.</span>
        </div>
        <div className="flex items-center gap-4">
          <Link href="https://twitter.com/" target="_blank" rel="noopener" aria-label="Twitter">
            <FaTwitter className="text-xl hover:text-[#1da1f2] transition-colors" />
          </Link>
          <Link href="https://github.com/" target="_blank" rel="noopener" aria-label="GitHub">
            <FaGithub className="text-xl hover:text-gray-400 transition-colors" />
          </Link>
        </div>
      </div>
    </footer>
  );
};

export default Footer; 