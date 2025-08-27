"use client";

import Link from "next/link";
import { ReactNode, ButtonHTMLAttributes } from "react";

interface StarBorderProps {
  href?: string;
  children: ReactNode;
  className?: string;
  onClick?: () => void;
}

export function StarBorder({
  href,
  children,
  className,
  onClick,
  ...props
}: StarBorderProps & ButtonHTMLAttributes<HTMLButtonElement>) {
  if (href) {
    // Renders as Next.js Link
    return (
      <Link
        href={href}
        className={`px-6 py-3 rounded-lg border border-slate-400 bg-white hover:bg-slate-100 ${className || ""}`}
      >
        {children}
      </Link>
    );
  }

  // Renders as button
  return (
    <button
      onClick={onClick}
      className={`px-6 py-3 rounded-lg border border-slate-400 bg-white hover:bg-slate-100 ${className || ""}`}
      {...props}
    >
      {children}
    </button>
  );
}
