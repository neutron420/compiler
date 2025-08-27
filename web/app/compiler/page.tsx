"use client";
import React, { useState } from "react";
import Editor from "@monaco-editor/react";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";

import { NavBar } from "@/components/tubelight-navbar";
import { Footer7 } from "@/components/footer-7";
import { LanguageSelector, languageOptions, LanguageKey } from "@/components/LanguageSelector";
 import { OutputPanel } from "@/components/OutputPanel"; // Commented out due to missing module

import { Home, User, Briefcase, Play, Loader } from "lucide-react";

// --- Starter Code for each language ---
const starterCode: Record<LanguageKey, string> = {
  custom: `// Welcome to the Custom Language!\nfn fibonacci(n) {\n  if (n <= 1) {\n    return n;\n  }\n  return fibonacci(n - 1) + fibonacci(n - 2);\n}\n\nlet result = fibonacci(10);\nprintln("Fibonacci(10) is:", result);\n`,
  rust: `fn main() {\n    println!("Hello, world from Rust!");\n}`,
  python: `print("Hello, world from Python!")`,
  c: `#include <stdio.h>\n\nint main() {\n    printf("Hello, world from C!\\n");\n    return 0;\n}`,
};

export default function CompilerPage() {
  const [code, setCode] = useState<string>(starterCode.custom);
  const [selectedLanguage, setSelectedLanguage] = useState<LanguageKey>("custom");
  const [output, setOutput] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [executionTime, setExecutionTime] = useState<number | null>(null);

  const handleLanguageChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const langKey = event.target.value as LanguageKey;
    setSelectedLanguage(langKey);
    setCode(starterCode[langKey]);
    // Reset output when language changes
    setOutput(null);
    setError(null);
    setExecutionTime(null);
  };

  const handleSubmit = async () => {
    setIsLoading(true);
    setOutput(null);
    setError(null);
    setExecutionTime(null);

    try {
      const response = await fetch("http://localhost:8080/compile", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code, language: selectedLanguage }),
      });
      const result = await response.json();
      setOutput(result.result);
      setError(result.error);
      setExecutionTime(result.execution_time_ms);
    } catch (err) {
      setError("Failed to connect to the server. Please ensure it's running and try again.");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 flex flex-col">
      <NavBar items={[
          { name: "Home", url: "/", icon: Home },
          { name: "About", url: "/about", icon: User },
          { name: "Compiler", url: "/compiler", icon: Briefcase },
      ]} />
      
      <main className="flex-grow flex flex-col p-4">
        <PanelGroup direction="vertical" className="flex-grow rounded-lg overflow-hidden bg-white shadow-md border">
          <Panel defaultSize={65} minSize={20}>
            {/* --- Top Panel: Editor --- */}
            <div className="h-full flex flex-col">
              <div className="flex items-center justify-between p-3 bg-gray-50 border-b">
                <LanguageSelector selectedLanguage={selectedLanguage} onLanguageChange={handleLanguageChange} />
                <button
                  onClick={handleSubmit}
                  disabled={isLoading}
                  className="px-5 py-2 bg-green-600 text-white rounded-md font-semibold flex items-center justify-center disabled:bg-gray-400 disabled:cursor-not-allowed hover:bg-green-700 transition-colors"
                >
                  {isLoading ? (
                    <>
                      <Loader className="animate-spin mr-2 h-5 w-5" />
                      Running...
                    </>
                  ) : (
                    <>
                      <Play className="mr-2 h-5 w-5" />
                      Run
                    </>
                  )}
                </button>
              </div>
              <div className="flex-grow">
                <Editor
                  language={languageOptions[selectedLanguage].editorLanguage}
                  value={code}
                  onChange={(value) => setCode(value || "")}
                  theme="vs-light"
                  options={{ 
                    minimap: { enabled: false }, 
                    fontSize: 14, 
                    wordWrap: "on",
                    padding: { top: 16 } // This adds the top padding
                  }}
                />
              </div>
            </div>
          </Panel>
          <PanelResizeHandle className="h-2 bg-gray-200 hover:bg-black transition-colors" />
          <Panel defaultSize={35} minSize={15}>
            {/* --- Bottom Panel: Output --- */}
            <OutputPanel 
                output={output} 
                error={error} 
                executionTime={executionTime} 
                isLoading={isLoading} 
            />
          </Panel>
        </PanelGroup>
      </main>
      
      <Footer7 />
    </div>
  );
}