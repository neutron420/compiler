import { MessageLoading } from "@/components/message-loading";

export default function Loading() {
  return (
    <div className="flex h-screen w-full items-center justify-center bg-white">
      <MessageLoading />
    </div>
  );
}