// Add this directive at the top to make the entire file a Client Component
"use client"; 

import { Squares } from "@/components/squares-background";
import { Home, User, Briefcase } from 'lucide-react';
import { NavBar } from "@/components/tubelight-navbar";
import { StarBorder } from "@/components/star-border";
import { TestimonialsColumn } from "@/components/testimonials-columns-1";
import { TreeView } from "@/components/tree-view";
import { motion } from "framer-motion";
import React, { useState, useEffect } from "react";
import { Footer7 } from "@/components/footer-7"

// --- Navigation Bar Component ---
export function NavBarDemo() {
  const navItems = [
    { name: 'Home', url: '#', icon: Home },
    { name: 'About', url: '#', icon: User },
    { name: 'Code Editor', url: '#', icon: Briefcase },
  ];

  return <NavBar items={navItems} />;
}

// --- Button Component ---
export function StarBorderDemo() {
  return (
    <div className="space-y-8">
      <StarBorder>
        Go to the Code Editor 
      </StarBorder>
    </div>
  );
}

// --- TreeView Demo Component ---
const DemoOne = () => {
  const treeData = [
    {
      id: "1",
      label: "Documents",
      children: [
        {
          id: "1-1",
          label: "Projects",
          children: [
            { id: "1-1-1", label: "Project A.pdf" },
            { id: "1-1-2", label: "Project B.docx" },
            {
              id: "1-1-3",
              label: "Archive",
              children: [
                { id: "1-1-3-1", label: "Old Project.zip" },
                { id: "1-1-3-2", label: "Backup.tar" },
              ],
            },
          ],
        },
        {
          id: "1-2",
          label: "Reports",
          children: [
            { id: "1-2-1", label: "Monthly Report.xlsx" },
            { id: "1-2-2", label: "Annual Report.pdf" },
          ],
        },
      ],
    },
    {
      id: "2",
      label: "Downloads",
      children: [
        { id: "2-1", label: "setup.exe" },
        { id: "2-2", label: "image.jpg" },
        { id: "2-3", label: "video.mp4" },
      ],
    },
    {
      id: "3",
      label: "Desktop",
      children: [{ id: "3-1", label: "shortcut.lnk" }],
    },
  ];

  return (
    <>
      <div className="max-w-xl mx-auto w-full">
        <TreeView
          data={treeData}
          onNodeClick={(
            node: { id: string; label: string; children?: Array<{ id: string; label: string; children?: unknown[] }> }
          ) => console.log("Clicked:", node.label)}
          defaultExpandedIds={["1"]}
        />
      </div>
    </>
  );
};

export { DemoOne };

// --- Footer Demo Component ---
const FooterDemo = () => {
  return (
    <Footer7 />
  );
};

// --- Testimonials Data ---
const testimonials = [
  {
    text: "This ERP revolutionized our operations, streamlining finance and inventory. The cloud-based platform keeps us productive, even remotely.",
    image: "https://randomuser.me/api/portraits/women/1.jpg",
    name: "Briana Patton",
    role: "Operations Manager",
  },
  {
    text: "Implementing this ERP was smooth and quick. The customizable, user-friendly interface made team training effortless.",
    image: "https://randomuser.me/api/portraits/men/2.jpg",
    name: "Bilal Ahmed",
    role: "IT Manager",
  },
  {
    text: "The support team is exceptional, guiding us through setup and providing ongoing assistance, ensuring our satisfaction.",
    image: "https://randomuser.me/api/portraits/women/3.jpg",
    name: "Saman Malik",
    role: "Customer Support Lead",
  },
  {
    text: "This ERP's seamless integration enhanced our business operations and efficiency. Highly recommend for its intuitive interface.",
    image: "https://randomuser.me/api/portraits/men/4.jpg",
    name: "Omar Raza",
    role: "CEO",
  },
  {
    text: "Its robust features and quick support have transformed our workflow, making us significantly more efficient.",
    image: "https://randomuser.me/api/portraits/women/5.jpg",
    name: "Zainab Hussain",
    role: "Project Manager",
  },
  {
    text: "The smooth implementation exceeded expectations. It streamlined processes, improving overall business performance.",
    image: "https://randomuser.me/api/portraits/women/6.jpg",
    name: "Aliza Khan",
    role: "Business Analyst",
  },
  {
    text: "Our business functions improved with a user-friendly design and positive customer feedback.",
    image: "https://randomuser.me/api/portraits/men/7.jpg",
    name: "Farhan Siddiqui",
    role: "Marketing Director",
  },
  {
    text: "They delivered a solution that exceeded expectations, understanding our needs and enhancing our operations.",
    image: "https://randomuser.me/api/portraits/women/8.jpg",
    name: "Sana Sheikh",
    role: "Sales Manager",
  },
  {
    text: "Using this ERP, our online presence and conversions significantly improved, boosting business performance.",
    image: "https://randomuser.me/api/portraits/men/9.jpg",
    name: "Hassan Ali",
    role: "E-commerce Manager",
  },
];

const firstColumn = testimonials.slice(0, 3);
const secondColumn = testimonials.slice(3, 6);
const thirdColumn = testimonials.slice(6, 9);

// --- Testimonials Component (UPDATED with delay) ---
const Testimonials = () => {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 1500);
    return () => clearTimeout(timer);
  }, []); 

  if (!isVisible) {
    return null;
  }

  return (
    // CHANGED: Removed dark background and fixed spacing
    <section className="py-20 relative w-full">
      <div className="container z-10 mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.1, ease: [0.16, 1, 0.3, 1] }}
          viewport={{ once: true }}
          className="flex flex-col items-center justify-center max-w-[540px] mx-auto"
        >
          <div className="flex justify-center">
            {/* CHANGED: Text and border color to dark gray */}
            <div className="border border-slate-300 py-1 px-4 rounded-lg text-slate-600">Testimonials</div>
          </div>
          {/* CHANGED: Heading text to dark gray */}
          <h2 className="text-xl sm:text-2xl md:text-3xl lg:text-4xl xl:text-5xl font-bold tracking-tighter mt-5 text-slate-900">
            What our users say
          </h2>
          {/* CHANGED: Paragraph text to medium gray */}
          <p className="text-center mt-5 opacity-75 text-slate-600">
            See what our customers have to say about us.
          </p>
        </motion.div>

        <div className="flex justify-center gap-6 mt-10 [mask-image:linear-gradient(to_bottom,transparent,black_25%,black_75%,transparent)] max-h-[740px] overflow-hidden">
          <TestimonialsColumn testimonials={firstColumn} duration={15} />
          <TestimonialsColumn testimonials={secondColumn} className="hidden md:block" duration={19} />
          <TestimonialsColumn testimonials={thirdColumn} className="hidden lg:block" duration={17} />
        </div>
      </div>
    </section>
  );
};

// --- Main Page Component (Default Export) ---
export default function HomePage() {
  return (
    // CHANGED: Main background to a light gray
    <main className="relative w-full bg-slate-50">
      
      {/* --- Hero Section --- */}
      <div className="relative flex h-screen w-full flex-col items-center justify-center">
        {/* Render the Navbar at the top */}
        <NavBarDemo />
        
        {/* Background Squares */}
        <div className="absolute inset-0 z-0">
          <Squares
            direction="diagonal"
            speed={0.5}
            squareSize={30}
            // CHANGED: Props updated for a light background
            borderColor="#DDD"
            hoverFillColor="#EEE"
          />
        </div>
        
        {/* Centered Content */}
        <div className="relative z-10 flex flex-col items-center text-center">
          {/* CHANGED: Heading text to dark gray */}
          <h1 className="text-5xl font-bold text-slate-900">
            Code.Connect
          </h1>
          {/* CHANGED: Paragraph text to medium gray */}
          <p className="mt-4 text-lg text-slate-600">
            This is Your own Coding Playground
          </p>
          {/* Render the Button below the text */}
          <div className="mt-8">
              <StarBorderDemo />
          </div>
        </div>
      </div>
      
      {/* --- TreeView Demo Section --- */}
      <section className="py-20 relative w-full">
        <div className="container z-10 mx-auto">
          <div className="flex flex-col items-center justify-center max-w-[540px] mx-auto mb-10">
            <div className="flex justify-center">
              <div className="border border-slate-300 py-1 px-4 rounded-lg text-slate-600">File Explorer</div>
            </div>
            <h2 className="text-xl sm:text-2xl md:text-3xl lg:text-4xl xl:text-5xl font-bold tracking-tighter mt-5 text-slate-900">
              Your Code Structure
            </h2>
            <p className="text-center mt-5 opacity-75 text-slate-600">
              Navigate through your project files and folders.
            </p>
          </div>
          <DemoOne />
        </div>
      </section>
      
      <Testimonials />
      
      {/* --- Footer Section --- */}
      <FooterDemo />
      
    </main>
  );
}