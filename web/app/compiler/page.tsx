"use client";
import React, { useState } from "react";
import { NavBar } from "@/components/tubelight-navbar";
import { Home, User, Briefcase } from "lucide-react";
import { Footer7 } from "@/components/footer-7";

export function NavBarDemo() {
  const navItems = [
    { name: "Home", url: "/", icon: Home },
    { name: "About", url: "/about", icon: User },
    { name: "Code Compiler", url: "/compiler", icon: Briefcase },
  ];

  return <NavBar items={navItems} />;
}

export default function CompilerPage() {
  const [code, setCode] = useState("");
  const [output, setOutput] = useState("");
  const [error, setError] = useState("");

  const handleCodeChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setCode(event.target.value);
  };

  const handleSubmit = async () => {
    setOutput("");
    setError("");

    try {
      const response = await fetch("http://localhost:8080/compile", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ code }),
      });

      const result = await response.json();

      if (result.error) {
        setError(result.error);
      } else {
        setOutput(result.result);
      }
    } catch (err) {
      setError("An error occurred while compiling the code.");
    }
  };

  return (
    <div className="min-h-screen bg-white">
      <NavBarDemo />
      <div className="max-w-4xl mx-auto px-6 py-16">
        <header className="mb-16">
          <div className="text-center">
            <h1 className="text-5xl font-bold text-black mb-4">
              Code.Connect Compiler
            </h1>
            <p className="text-xl text-gray-600">
              Write, compile, and run your code online.
            </p>
            <div className="w-24 h-1 bg-black mx-auto mt-6"></div>
          </div>
        </header>

        <section className="mb-16">
          <div className="bg-gray-50 border border-gray-200 rounded-lg p-8">
            <textarea
              className="w-full h-64 p-4 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-black"
              value={code}
              onChange={handleCodeChange}
              placeholder="Enter your code here..."
            ></textarea>
            <button
              className="mt-4 px-6 py-3 bg-black text-white rounded-lg hover:bg-gray-800 transition-colors font-medium"
              onClick={handleSubmit}
            >
              Compile & Run
            </button>
          </div>
        </section>

        <section>
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-black mb-4">Output</h2>
            <div className="w-16 h-0.5 bg-black mx-auto"></div>
          </div>
          <div className="bg-gray-50 border border-gray-200 rounded-lg p-8">
            {output && (
              <pre className="text-lg text-gray-700 leading-relaxed">
                {output}
              </pre>
            )}
            {error && (
              <pre className="text-lg text-red-500 leading-relaxed">
                {error}
              </pre>
            )}
          </div>
        </section>
      </div>
      <Footer7 />
    </div>
  );
}