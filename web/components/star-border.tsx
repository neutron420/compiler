"use client";
import { cn } from "@/lib/utils"
import { ElementType, ComponentPropsWithoutRef } from "react"

interface StarBorderProps<T extends ElementType> {
  as?: T
  color?: string
  speed?: string
  className?: string
  children: React.ReactNode
}

export function StarBorder<T extends ElementType = "button">({
  as,
  className,
  color,
  speed = "6s",
  children,
  ...props
}: StarBorderProps<T> & Omit<ComponentPropsWithoutRef<T>, keyof StarBorderProps<T>>) {
  const Component = as || "button"
  // CHANGED: Star color defaults to white for visibility on a black background
  const defaultColor = color || "#FFFFFF"

  return (
    <Component 
      className={cn(
        "relative inline-block py-[1px] overflow-hidden rounded-[20px]",
        className
      )} 
      {...props}
    >
      <div
        className={cn(
          "absolute w-[300%] h-[50%] bottom-[-11px] right-[-250%] rounded-full animate-star-movement-bottom z-0",
          // CHANGED: Adjusted opacity for a balanced look
          "opacity-50" 
        )}
        style={{
          background: `radial-gradient(circle, ${defaultColor}, transparent 10%)`,
          animationDuration: speed,
        }}
      />
      <div
        className={cn(
          "absolute w-[300%] h-[50%] top-[-10px] left-[-250%] rounded-full animate-star-movement-top z-0",
          // CHANGED: Adjusted opacity for a balanced look
          "opacity-50"
        )}
        style={{
          background: `radial-gradient(circle, ${defaultColor}, transparent 10%)`,
          animationDuration: speed,
        }}
      />
      <div className={cn(
        "relative z-1 border text-center text-base py-4 px-6 rounded-[20px]",
        // CHANGED: Swapped to a black background, white text, and subtle hover effect
        "bg-black text-white border-gray-800", 
        "hover:bg-gray-900 transition-colors duration-200", 
        "dark:bg-black dark:border-gray-800 dark:text-white dark:hover:bg-gray-900"
      )}>
        {children}
      </div>
    </Component>
  )
}