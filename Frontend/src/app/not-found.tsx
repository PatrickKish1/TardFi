import Link from "next/link";
import { Button } from "../components/ui/button";

export default function NotFound() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] text-white">
      <h1 className="text-5xl font-bold text-gradient mb-4">404</h1>
      <h2 className="text-2xl mb-2">Page Not Found</h2>
      <p className="mb-6 text-center max-w-md">
        Sorry, the page you are looking for does not exist or has been moved.
      </p>
      <Button asChild className="custom-gradient px-6 py-2 rounded-2xl">
        <Link href="/">Go Home</Link>
      </Button>
    </div>
  );
} 